-- Create signatures table
CREATE TABLE IF NOT EXISTS signatures (
    id SERIAL PRIMARY KEY,
    program_id VARCHAR(75) NOT NULL,
    signature VARCHAR(100) NOT NULL,
    slot BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    processed bool NOT NULL,
    UNIQUE (program_id, signature)
    );

