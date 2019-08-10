SELECT a, b, c, d
FROM
  tbl1
  JOIN tbl2 ON tbl1.id = tbl2.id
;
-- Since the resulting columns aren't expressed in terms of which table they're
-- coming from, it should just show the resulting table and pointing to the
-- tables that made it up
--
--
--   ________________
--   | a | b | c | d|
--   ----------------
--       /       \
--      /         \
--   ______      ______
--   |tbl2|      |tbl2|
--   ------      ------
