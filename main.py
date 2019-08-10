#!/usr/bin/env python

import sys
import json
import psqlparse
import os

from graphviz import Digraph

# Input can be a file or direct sql statement
inp = sys.argv[1] or ""
query = inp

if os.path.isfile(inp):
    with open(inp, "r") as f:
        query = f.read()

res = psqlparse.parse(query)[0]._obj
if not res:
    raise Exception("No result from psqlparse")

g = Digraph("g", format="png", filename="output.gv", node_attr={"shape": "record"})

# Create nodes from tables
# @TODO recurse when Subselect


# Result Columns
fields = []
for i, field in enumerate(res["targetList"]):
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
    val = field["ResTarget"]["val"]["ColumnRef"]["fields"][0]["A_Star"]
    fields.append(f"<f{i}> All")
g.node("result", "|".join(fields))

# Source Tables
frm = res["fromClause"][0]["RangeVar"]["relname"]
g.node("from", f"<f0> {frm}")
g.edges([('result:f0', 'from:f0')])



# Output
print(g.source)
g.view()
