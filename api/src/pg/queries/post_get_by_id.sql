SELECT
    id AS "id: PostId",
    title,
    content
FROM posts
WHERE id = $1
