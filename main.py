#!/usr/bin/env python

import pdb
import itertools
import json
import os
import sys
from uuid import uuid4

import psqlparse
from graphviz import Digraph

from graph_types import Column, Table

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


def handle_cols(target_list, output, tables=None):
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

        # Only add columns to table if they don't already exist on it
        # if c.name not in [x.name for x in tables[output].columns]:
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
            # tables[ancestor_tbl_name].columns.append(ancestor_col)
            c.came_from = (tables[ancestor_tbl_name], ancestor_col)
        else:
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


def name_tbl(tbl, tables=None):
    """
    Create a Table object and place it into the map of Tables.

    Args:
        tbl (dict): A `RangeVar` shape as per the parser
        tables (dict): Map of currently known tables. Dict is mutated.
    """
    use_alias = tbl.get("alias", {}).get("Alias", {}).get("aliasname", None)
    t_name = use_alias if use_alias else tbl["relname"]
    tables[t_name] = Table(t_name)
    return t_name


def handle_from_clause(res, output, tables=None):
    """
    Pull out the tables from the parser output structure

    Args:
        res (list): Response structure from the parser
        output (str): The name of the output table where tables will be added
        tables (dict): Map of currently known tables. Dict is mutated.
    """
    if type(res) == dict:
        res = res.get("fromClause", res)

    for frm in res:
        # Get to the lowest grain first

        if frm.get("RangeSubselect", None):
            tbl = frm.get("RangeSubselect")
            new_tbl_name = name_tbl(tbl, tables)
            handle_select_stmt(
                tbl["subquery"]["SelectStmt"],
                new_tbl_name,
                tables={new_tbl_name: tables[new_tbl_name]},
            )
        elif frm.get("JoinExpr", None):
            tbl = frm.get("JoinExpr", None)
            joined_tables = list(map(lambda x: tbl.get(x), ["larg", "rarg"]))
            handle_from_clause(joined_tables, output, tables)
        else:
            tbl = frm.get("RangeVar", None)
            assert tbl is not None
            name_tbl(tbl, tables)


def handle_select_stmt(res, output_name=None, tables=None):
    """
    Create tables and populate them with columns from parser's output

    Args:
        res (dict): The response structure from the parser
        tables (dict): Map of currently known tables. Dict is mutated.
    """
    if output_name is None:
        output_name = f"output-{str(uuid4())[:8]}"

    tables = tables if tables else {}
    actual_table = tables.get(output_name, Table(output_name))
    tables[output_name] = actual_table

    handle_from_clause(res, output_name, tables)
    handle_cols(res["targetList"], output_name, tables)
    return tables


# PROCESS
tables = handle_select_stmt(res, "output", {})
print(tables)


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
