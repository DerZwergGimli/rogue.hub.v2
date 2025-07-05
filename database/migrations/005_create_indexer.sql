CREATE SCHEMA indexer;

CREATE TABLE IF NOT EXISTS indexer.programs (
    program_id VARCHAR(50) NOT NULL PRIMARY KEY
);


CREATE TABLE IF NOT EXISTS indexer.signatures (
    signature VARCHAR(88) NOT NULL PRIMARY KEY,
    slot BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);


CREATE TABLE IF NOT EXISTS indexer.program_signatures (
    PRIMARY KEY (program_id, signature),
    program_id VARCHAR(50) REFERENCES indexer.programs(program_id) ON DELETE CASCADE,
    signature VARCHAR(88) REFERENCES indexer.signatures(signature) ON DELETE CASCADE,
    processed BOOL NOT NULL
);


CREATE TYPE indexer_direction_type AS ENUM ('UP', 'DOWN');


CREATE TABLE IF NOT EXISTS indexer.indexer (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    direction indexer_direction_type,
    program_id VARCHAR(50) REFERENCES indexer.programs(program_id) ON DELETE CASCADE,
    signature VARCHAR(88) REFERENCES indexer.signatures(signature) ON DELETE CASCADE,
    block BIGINT,
    timestamp TIMESTAMPTZ,
    finished BOOLEAN,
    fetch_limit INTEGER NOT NULL
);
