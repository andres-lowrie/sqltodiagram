#!/usr/bin/env python

import sys
import json
import psqlparse


query = sys.argv[1] or ''
res = psqlparse.parse(query)

if res:
    print(json.dumps(res[0]._obj))
