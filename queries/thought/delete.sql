DELETE FROM thought
WHERE id = $1
RETURNING *