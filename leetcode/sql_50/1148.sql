SELECT
  DISTINCTauthor_id AS id
FROM
  Views
WHERE
  author_id = viewer_id
ORDER BY
  idASC;
-- 評価順の話
-- FROM
-- ON
-- JOIN
-- WHERE
-- GROUP BY
-- HAVING
-- SELECT
-- DISTINCT
-- ORDER BY
-- TOP（LIMIT）
-- distinct, order by, limitはselectしてから!!(なのでselectでつけたidという別名が使える)
-- order byは当たり前(ソートするので取得後)
-- distinctも取得してから重複を消す
-- limitも数の制限なので先に条件に該当するレコードを取ってきてからその数だけ返す
