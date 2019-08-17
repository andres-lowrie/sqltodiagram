SELECT a, b
FROM tbl1
WHERE
  a IN (SELECT c FROM tbl2)
;
