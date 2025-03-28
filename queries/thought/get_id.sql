SELECT
    id,
    content,
    created_at,
    updated_at
FROM thought
WHERE id = $1