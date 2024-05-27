CREATE TABLE IF NOT EXISTS ftd_account
(
    address                          VARCHAR(64) PRIMARY KEY,
    created_at                       TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at                       TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX IF NOT EXISTS ftd_account_idx_address_trgm
    ON ftd_account USING GIN (address gin_trgm_ops);