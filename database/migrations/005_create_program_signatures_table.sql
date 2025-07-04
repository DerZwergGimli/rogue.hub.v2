CREATE SCHEMA indexer;

CREATE TABLE IF NOT EXISTS indexer.programs (
    program_id VARCHAR(75) NOT NULL PRIMARY KEY
);


CREATE TABLE IF NOT EXISTS indexer.signatures (
    signature VARCHAR(100) NOT NULL PRIMARY KEY,
    slot BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);


CREATE TABLE IF NOT EXISTS indexer.program_signatures (
    PRIMARY KEY (program_id, signature),
    program_id VARCHAR(75) REFERENCES indexer.programs(program_id) ON DELETE CASCADE,
    signature VARCHAR(100) REFERENCES indexer.signatures(signature) ON DELETE CASCADE,
    processed BOOL NOT NULL
);



CREATE TABLE IF NOT EXISTS indexer.indexer (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50),
    program_id VARCHAR(75) REFERENCES indexer.programs(program_id) ON DELETE CASCADE,
    start_signature VARCHAR(100) REFERENCES indexer.signatures(signature) ON DELETE CASCADE,
    before_signature VARCHAR(100) REFERENCES indexer.signatures(signature) ON DELETE CASCADE,
    start_block BIGINT,
    before_block BIGINT,
    finished BOOLEAN
);







