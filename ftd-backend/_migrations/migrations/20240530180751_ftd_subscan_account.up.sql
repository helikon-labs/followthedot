CREATE TABLE IF NOT EXISTS ftd_subscan_account
(
    id                          SERIAL PRIMARY KEY,
    address                     VARCHAR(64)                 NOT NULL,
    display                     VARCHAR(2048),
    account_index               VARCHAR(2048),
    account_display             VARCHAR(2048),
    account_identity            BOOLEAN,
    parent_address              VARCHAR(64),
    parent_display              VARCHAR(2048),
    parent_sub_symbol           VARCHAR(2048),
    parent_identity             BOOLEAN,
    merkle_science_address_type VARCHAR(2048),
    merkle_science_tag_type     VARCHAR(2048),
    merkle_science_tag_sub_type VARCHAR(2048),
    merkle_science_tag_name     VARCHAR(2048),
    created_at                  TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at                  TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT ftd_subscan_account_u_address UNIQUE (address)
);

CREATE INDEX IF NOT EXISTS ftd_subscan_account_idx_address
    ON ftd_subscan_account USING GIN (address gin_trgm_ops);