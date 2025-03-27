CREATE TABLE thought (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    text VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

SELECT manage_updated_at('thought');