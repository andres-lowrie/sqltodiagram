#!/usr/bin/env python

import sys
import json
import psqlparse
import os


# Input can be a file or direct sql statement
inp = sys.argv[1] or ''
query = inp

if os.path.isfile(inp):
    with open(inp, 'r') as f:
        query = f.read()

res = psqlparse.parse(query)

if res:
    print(json.dumps(res[0]._obj))
