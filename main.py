#!/usr/bin/env python

import itertools
import sys
import json
import psqlparse
import os

from sql_types import Table, Column

from graphviz import Digraph

# Input can be a file or direct sql statement
# @TODO use argparse
inp = sys.argv[1] or ""
preview = True if sys.argv[2] == "y" else False
query = inp

if os.path.isfile(inp):
    with open(inp, "r") as f:
        query = f.read()

res = psqlparse.parse(query)[0]._obj
if not res:
    raise Exception("No result from psqlparse")


def handle_cols(target_list):
    """
    Traverse the columns in the output table (the otter most table) and see if we can link up columns to the table
    references.

    @TODO: Remove dependency on global "tables"
    @TODO: Create types for the maps the parser outputs?

    Args:
        target_list (list): List of `ResTarget`s as per the parser
    """
    for res_target in target_list:
        # The number of fields corresponds to the `dots` in the column name passed into the SqlStatement ie:
        #     col_name
        #     tbl.col_name
        #     tbl.*
        #
        # The shapes of the "field" strucutre look like this:
        #
        # When just the col_name is used:
        #     ```
        #     {
        #         "String": {"str": "col_name"}
        #     }
        #     ```
        #
        # When an asterisk is used:
        #     ```
        #     {
        #         "A_Star": {}
        #     }
        #     ```
        #
        # So if the Fully Qualified Name (fqn) of a column is used then each 2 fields are returned; the left
        # side of the dot ie. the table name and the right hand side of the dot, the column name
        #
        # When an fqn is used:
        #     ```
        #     {
        #         "String": {"str": "table_name"}
        #     },
        #     {
        #         "String": {"str": "col_name"}
        #     }
        #     ```
        target = res_target["ResTarget"]
        fields = target["val"]["ColumnRef"]["fields"]
        using_fqn = True if len(fields) > 1 else False

        # What should we label the column?
        #
        # The `Column` type will handle naming the column when an asterisk is used so we don't have to come up with a
        # name here we only need to identify that an asterisk is used
        is_star = False

        # When an alias is passed, the parser makes that name available directly on the `ResTarget` shape
        name = target.get("name", None)
        if name is None:
            name_field = fields[-1].get("String", None)

            # Handle the "select all from table" syntax ie: "tbl.*"
            is_star = False if name_field else True
            if name_field:
                name = name_field["str"]

        c = Column(name, is_star=is_star, table=tables["output"])
        tables["output"].columns.append(c)

        # Figure out where this column came from.
        if using_fqn is True:
            ancestor_tbl_name = fields[0].get("String").get("str")
            # In order to make a more specific graph, we'll copy this column onto the ancestor table so that the edge
            # can point to it
            ancestor_col = Column(
                name, is_star=is_star, table=tables[ancestor_tbl_name]
            )
            tables[ancestor_tbl_name].columns.append(ancestor_col)
            c.came_from = (tables[ancestor_tbl_name], ancestor_col)
        else:
            # If the sql stmt didn't use the `tbl.col` syntax then the best we can do is have the output table point to the
            # tables outlined in the `from` section of the sql statement
            #
            # We'll only want to draw one line between columns and tables to keep the graph readable
            other_tables = [
                tables[k]
                for k, v in tables.items()
                if k != "output"
                and k not in [t.name for t in tables["output"].table_links]
            ]
            tables["output"].table_links.extend(other_tables)


def handle_tbl(tbl):
    """
    Create a Table object and place it into the map of Tables.

    Args:
        tbl (dict): A `RangeVar` shape as per the parser
    """
    t_name = tbl["relname"]
    tables[t_name] = Table(t_name)


def handle_from_clause(res):
    """
    Pull out the tables from the parser output structure

    Args:
        res (dict): The response structure from the parser
    """
    for frm in res["fromClause"]:
        # Not all from statements are simple "RangeVar"s and thus we'll need to check expression type in order to see
        # where in the strucutre the `RangeVar` (ie Table) is
        tbl = frm.get("RangeVar", None)

        if tbl is None:
            # @TODO This could also be SubQuery style. I think we it can be either a Join or
            # SubQuery and those can hold each other within themselves...need to check postgres manual to confrim
            tbl = frm.get("JoinExpr", None)
            assert tbl is not None, "@TODO: support this query"

            joined_tables = map(lambda x: tbl.get(x).get("RangeVar"), ["larg", "rarg"])
            [handle_tbl(t) for t in joined_tables]
            return

        t_name = tbl["relname"]
        tables[t_name] = Table(t_name)


# @TODO wrap this process in a function so we can call it recursively since the structure parser (and the nature of sql
# as I understand it) is recursive
# OUTPUT
# The `output` referes to the table that the select statement is creating.
tables = {"output": Table("output")}

# Create all the tables the query references
handle_from_clause(res)

# Add the selected columns to the output table
handle_cols(res["targetList"])


# RENDERING
g = Digraph("g", format="png", filename="output.gv", node_attr={"shape": "record"})
for t in tables.values():
    g.node(
        t.name,
        "{" + t.repre + "|{" + "|".join(map(lambda c: c.repre, t.columns)) + "}" + "}",
    )
    # make edges
    for tl in t.table_links:
        g.edge(f"{t.name}:{t.id}", f"{tl.name}:{tl.id}")

    for c in t.columns:
        if c.came_from:
            rt, rc = c.came_from
            g.edge(f"{t.name}:{c.id}", f"{rt.name}:{rc.id}")

# # Output
print(g.source)
if preview:
    g.view()
