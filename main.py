#!/usr/bin/env python

import itertools
import json
import os
import sys
from uuid import uuid4

import psqlparse
from graphviz import Digraph

from sql_types import Column, Table

# Input can be a file or direct sql statement
# @TODO use argparse
inp = sys.argv[1] or ""
preview = True if sys.argv[2] == "y" else False
dump_sql = True if sys.argv[3] == "y" else False
query = inp

if os.path.isfile(inp):
    with open(inp, "r") as f:
        query = f.read()

res = psqlparse.parse(query)[0]._obj
if not res:
    raise Exception("No result from psqlparse")

if dump_sql:
    print(json.dumps(res))
    sys.exit(0)


def handle_cols(target_list, output, tables=None, with_links=True):
    """
    Traverse the columns in the output table (the otter most table) and see if we can link up columns to the table
    references.

    @TODO: Create types for the maps the parser outputs?

    Args:
        target_list (list): List of `ResTarget`s as per the parser
        output (str): The name of the output table where columns will be added
        tables (dict): Map of currently known tables. Dict is mutated.
        with_links (bool): Will skip linking the columns to all known tables if set
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

        c = Column(name, is_star=is_star, table=tables[output])
        tables[output].columns.append(c)

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
        elif with_links:
            # If the sql stmt didn't use the `tbl.col` syntax then the best we can do is have the output table point to the
            # tables outlined in the `from` section of the sql statement
            #
            # We'll only want to draw one line between columns and tables to keep the graph readable
            other_tables = [
                tables[k]
                for k, v in tables.items()
                if k != output and k not in [t.name for t in tables[output].table_links]
            ]
            tables[output].table_links.extend(other_tables)


def handle_tbl(tbl, tables=None):
    """
    Create a Table object and place it into the map of Tables.

    Args:
        tbl (dict): A `RangeVar` shape as per the parser
        tables (dict): Map of currently known tables. Dict is mutated.
    """
    t_name = tbl["relname"]
    tables[t_name] = Table(t_name)


def handle_where_clause():
    raise NotImplementedError()


def handle_joins(res, output, tables=None):
    tbl = res.get("RangeVar", None)
    if tbl:
        return handle_tbl(tbl, tables)

    tbl = res.get("RangeSubselect", None)
    assert (
        tbl is not None
    ), "Expecting 'JoinExpr' to have either a RangeVar to RangeSubselect"

    stmt = tbl.get("subquery", {}).get("SelectStmt", None)
    assert stmt is not None, "Could not find subquery.SelectStmt in JoinExpr"

    # @TODO instead of mutating data, change the handle_from_clause to accepct different types of dicts that resemble a
    # table
    #
    # Mutate the nested shape to that function that makes table can handle it as written
    alias = tbl.get("alias", {}).get("Alias", {}).get("aliasname", None)
    if alias:
        for t in stmt["fromClause"]:
            t["RangeVar"]["relname"] = alias

    return handle_select_stmt(stmt, alias, tables, with_links=False)


def handle_from_clause(res, output, tables=None):
    """
    Pull out the tables from the parser output structure

    Args:
        res (list): Response structure from the parser
        output (str): The name of the output table where tables will be added
        tables (dict): Map of currently known tables. Dict is mutated.
    """
    # Not all from statements are simple "RangeVar"s and thus we'll need to check expression type in order to see
    # where in the strucutre the `RangeVar` (ie Table) is
    for frm in res:
        tbl = frm.get("RangeVar", None)

        if tbl is None:
            # @TODO This could also be SubQuery style. I think it can be either a Join or
            # SubQuery and those can hold each other within themselves...need to check postgres manual to confrim
            tbl = frm.get("JoinExpr", None)
            assert tbl is not None, "Expecting the dict to have a key 'JoinExpr'"

            joined_tables = map(lambda x: tbl.get(x), ["larg", "rarg"])
            [handle_joins(t, output, tables) for t in joined_tables]
            return

        use_alias = tbl.get("alias", {}).get("Alias", {}).get("aliasname", None)
        t_name = use_alias if use_alias else tbl["relname"]
        tables[t_name] = Table(t_name)


def handle_select_stmt(res, output_name=None, tables=None, with_links=True):
    """
    Create tables and populate them with columns from parser's output

    Args:
        res (dict): The response structure from the parser
        tables (dict): Map of currently known tables. Dict is mutated.
        with_links (bool): Passed to handle_cols
    """
    if output_name is None:
        output_name = f"output-{str(uuid4())[:8]}"

    tables = tables if tables else {}
    actual_table = tables.get(output_name, Table(output_name))
    tables[output_name] = actual_table

    handle_from_clause(res.get("fromClause"), output_name, tables)
    handle_cols(res["targetList"], output_name, tables, with_links)
    return tables


# PROCESS
tables = handle_select_stmt(res, "output", {})


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
