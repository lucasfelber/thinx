UPDATE thought
SET content = $2
WHERE id = $1
RETURNING *