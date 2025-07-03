-- Create indexer table
CREATE TABLE IF NOT EXISTS indexer (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50),
    program_id VARCHAR(75),
    start_signature VARCHAR(100)  NOT NULL,
    before_signature VARCHAR(100),
    start_block BIGINT,
    before_block BIGINT,
    finished BOOLEAN
);

