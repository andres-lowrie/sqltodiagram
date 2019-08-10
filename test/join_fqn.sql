SELECT tbl1.a, tbl1.b, tbl2.c, tbl2.d
FROM
  tbl1
  JOIN tbl2 ON tbl1.id = tbl2.id
;
-- It should just show the resulting table and pointing to the cloumns from the
-- tables that make it up
--
--
--   ________________
--   | a | b | c | d|
--   ----------------
--     |  /     \   \
--     | /       \   \
--   _________     _________
--   |  tbl2 |     |  tbl2 |
--   |=======|     |=======|
--   | a | b |     | c | d |
--   ---------     ---------
