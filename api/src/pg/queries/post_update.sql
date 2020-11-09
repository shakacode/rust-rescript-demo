UPDATE posts
SET
    title = $2,
    content = $3
WHERE id = $1
RETURNING
    id AS "id: PostId",
    title,
    content
