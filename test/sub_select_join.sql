SELECT a, b, c, foo.d
FROM
  (SELECT d FROM tbl2) AS foo
  JOIN bar ON foo.id = bar.id
;
-- It should be able to pick out that `d` comes from the `foo` subselect while
-- the other columns come from `bar`
--
--
--     _________________
--     | a | b | c | d |
--     -----------------
--       |  |    /     \
--       |  |   /       \
--     ____________      _______
--     |   bar    |      | foo |
--     |==========|      |=====|
--     | a | b | c|      |  d  |
--     ------------      -------
