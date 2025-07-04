CREATE SCHEMA staratlas;

CREATE TABLE IF NOT EXISTS staratlas.tokens
(
    id         SERIAL PRIMARY KEY,
    mint       VARCHAR(50) NOT NULL,
    name       VARCHAR(50),
    symbol     VARCHAR(10),
    token_type VARCHAR(50)

    );


CREATE TABLE IF NOT EXISTS staratlas.players (
    id SERIAL PRIMARY KEY,
    wallet_address VARCHAR(50) NOT NULL UNIQUE,
    username VARCHAR(50),
    first_seen   TIMESTAMPTZ NOT NULL,
    last_active TIMESTAMPTZ NOT NULL
    );

CREATE INDEX IF NOT EXISTS idx_players_wallet_address ON staratlas.players(wallet_address);
CREATE INDEX IF NOT EXISTS idx_players_last_active ON staratlas.players(last_active);