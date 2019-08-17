select a as foo_a from tbl1 as t;
-- It should prefer the alias naming
--       _______
--       |foo_a|
--       -------
--          |
--       ______
--       |tbl1|
--       ------
