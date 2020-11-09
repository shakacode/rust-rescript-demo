INSERT INTO posts (
    title,
    content
)
VALUES ($1, $2)
RETURNING
    id AS "id: PostId",
    title,
    content
