CREATE SCHEMA market;


CREATE TABLE market.exchanges
(
    id        SERIAL PRIMARY KEY,
    slot      INTEGER                         NOT NULL,
    signature VARCHAR(88)                    NOT NULL,
    index     integer                         NOT NULL,
    timestamp      TIMESTAMPTZ                     NOT NULL,
    side      VARCHAR(4)                      NOT NULL,
    buyer     INTEGER REFERENCES staratlas.players (id) NOT NULL,
    seller    INTEGER REFERENCES staratlas.players (id) NOT NULL,
    asset     INTEGER REFERENCES staratlas.tokens (id)  NOT NULL,
    pair      INTEGER REFERENCES staratlas.tokens (id)  NOT NULL,
    price     DOUBLE PRECISION                NOT NULL,
    size      INTEGER                         NOT NULL,
    volume    DOUBLE PRECISION                NOT NULL,
    fee       DOUBLE PRECISION                NOT NULL,
    buddy     DOUBLE PRECISION                NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_marketplace_exchanges_timestamp ON market.exchanges (timestamp);
CREATE INDEX IF NOT EXISTS idx_marketplace_exchanges_side ON market.exchanges (side);
CREATE INDEX IF NOT EXISTS idx_marketplace_exchanges_asset_pair ON market.exchanges (asset, pair);
CREATE INDEX IF NOT EXISTS idx_marketplace_exchanges_buyer_id ON market.exchanges (buyer);
CREATE INDEX IF NOT EXISTS idx_marketplace_exchanges_seller_id ON market.exchanges (seller);

ALTER TABLE market.exchanges
    ADD CONSTRAINT unique_txhash_index UNIQUE (signature, index);

