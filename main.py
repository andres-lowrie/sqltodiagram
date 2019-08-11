#!/usr/bin/env python

import itertools
import sys
import json
import psqlparse
import os

from sql_types import Table, Column

from graphviz import Digraph

def flatmap(func, *iterable):
    return itertools.chain.from_iterable(map(func, *iterable))

# Input can be a file or direct sql statement
inp = sys.argv[1] or ""
preview = True if sys.argv[2] == "y" else False
query = inp

if os.path.isfile(inp):
    with open(inp, "r") as f:
        query = f.read()

res = psqlparse.parse(query)[0]._obj
if not res:
    raise Exception("No result from psqlparse")

g = Digraph("g", format="png", filename="output.gv", node_attr={"shape": "record"})


# At this point we'll know that we will always have an output table because the parser would've failed by now
tables = {}


def handle_tbl(tbl, target_list, resolve_names=False):
    t_name = tbl["relname"]
    tables[t_name] = Table(t_name)

    # Now we can traverse the columns in the output table (the otter most table) and see if we have any columns that we can
    # link up
    for res_target in target_list:
        # @NOTE
        # The number of fields corresponds to the `dots` in the column name so if use the fqn of a column then each
        # ColumnRef will have 2 fields; the left side of the dot ie: the table name and the right hand side of the dot , the
        # column name
        #
        # When an "*" is passed as select the field shape changes, it's:
        #     "A_Star": {}
        #
        # The typical shape will be 1 item, with a dict keyed like "String" which holds a field "str" that is the actual
        # name we want
        #     ["fields"][0]["String"]["str"]

        target = res_target["ResTarget"]
        fields = target["val"]["ColumnRef"]["fields"]
        using_fqn = True if len(fields) > 1 else False

        # What should we label the column?
        #
        # The Column type will also handle naming the column if an asterisk is used so we don't have to come up with a
        # name here
        is_star = False
        # If an alias was passed then we can just use that directly, if not we'll have to check the fields list to
        # determine a name
        name = target.get("name", None)
        if name is None:
            # Select syntax is:
            #     col_name
            #     tbl.col_name
            #     tbl.*
            # Figure out which one

            # The parser will return the right side of the select syntax as the last item in the list
            id_field = fields[-1]
            name_field = id_field.get("String", None)

            # Handle the "select all from table" syntax
            is_star = False if name_field else True
            if name_field:
                name = name_field["str"]

        c = Column(name, is_star=is_star, table=tables[t_name])
        # Figure out where this column came from.
        # If the sql stmt didn't use the `tbl.col` syntax then the best we can do is point to the table
        if resolve_names:
            if using_fqn is False:
                c.came_from = tables[t_name]

        tables[t_name].columns.append(c)


# The outter most `From` statement is where we can start identifying where the columns in the output table are coming
# from
def handle_sql_stmt(res):
    for frm in res["fromClause"]:
        # @TODO not all from statements are simple "RangeVar"s and thus we'll need to check expression type in order to see
        # where in the strucutre the rangevar (ie table) is
        tbl = frm.get("RangeVar", None)

        if tbl is None:
            # Then we're dealing with a JOIN expression
            tbl = frm.get("JoinExpr", None)
            assert tbl is not None, "@TODO: support this query"

            # @TODO
            # JoinExpr will contain 2 tables, a left table and a right one. We need to check each of those tables to see if
            # the output table has columns that reference them
            joined_tables = map(lambda x: tbl.get(x).get("RangeVar"), ["larg", "rarg"])
            [handle_tbl(t, res.get("targetList")) for t in joined_tables]
            return

        handle_tbl(tbl, res.get("targetList"))


# OUTPUT

# Parse the statement
handle_sql_stmt(res)

# Create the output table
tables["output"] = Table("output")
handle_tbl({"relname": "output"}, res["targetList"], resolve_names=True)

columns = list(flatmap(lambda c: c, [t.columns for t in tables.values()]))

# print(tables["output"].columns[0].came_from)
for t in tables.values():
    print("table", t, t.name)
    print(t.name)
    # [print(i.name) for i in t.columns]
    g.node(
        t.name,
        "{" + t.repre + "|{" + "|".join(map(lambda c: c.repre, t.columns)) + "}" + "}",
    )

    # Create edges
    for c in t.columns:
        if c.came_from:
            g.edge(f"{c.table.name}:{c.id}", f"{c.came_from.name}:{c.came_from.id}")
        else:
            for rc in columns:
                if rc.id is not c.id:
                    g.edge(f"{c.table.name}:{c.id}", f"{c.came_from.name}:{c.came_from.id}")

        # print(c.came_from)
# g.node(output_tbl.name, "|".join(fields))

# # Source Tables
# g.node(frm.name, frm.repre)
# g.edges([("result:f0", "from:f0")])


# # Output
print(g.source)
if preview:
    g.view()
