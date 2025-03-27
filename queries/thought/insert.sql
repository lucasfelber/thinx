INSERT INTO thought (
    content
)
VALUES (
    $1
)
RETURNING
    id,
    content,
    created_at,
    updated_at