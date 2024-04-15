CREATE TABLE IF NOT EXISTS ftd_block
(
    hash                VARCHAR(64) PRIMARY KEY,
    number              BIGINT NOT NULL,
    timestamp           BIGINT NOT NULL,
    parent_hash         VARCHAR(66) NOT NULL,
    created_at          TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS ftd_block_idx_number
    ON ftd_block (number);
CREATE INDEX IF NOT EXISTS ftd_block_idx_timestamp
    ON ftd_block (timestamp);