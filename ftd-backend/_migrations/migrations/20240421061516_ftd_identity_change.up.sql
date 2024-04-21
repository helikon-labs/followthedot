CREATE TABLE IF NOT EXISTS ftd_identity_change
(
    id                    SERIAL PRIMARY KEY,
    block_hash            VARCHAR(64)                 NOT NULL,
    block_number          BIGINT                      NOT NULL,
    timestamp             BIGINT                      NOT NULL,
    extrinsic_index       INTEGER                     NOT NULL,
    extrinsic_event_index INTEGER                     NOT NULL,
    event_index           INTEGER                     NOT NULL,
    address               VARCHAR(64)                 NOT NULL,
    display               VARCHAR(256),
    legal                 VARCHAR(256),
    web                   VARCHAR(256),
    riot                  VARCHAR(128),
    email                 VARCHAR(128),
    twitter               VARCHAR(128),
    judgement             VARCHAR(64),
    super_address         VARCHAR(64),
    sub_display           VARCHAR(256),
    created_at            TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT ftd_identity_change_u_block_hash_extrinsic_index_event_index UNIQUE (block_hash, extrinsic_index, event_index),
    CONSTRAINT ftd_identity_change_fk_block_hash
        FOREIGN KEY (block_hash)
            REFERENCES ftd_block (hash)
            ON DELETE RESTRICT
            ON UPDATE CASCADE,
    CONSTRAINT ftd_identity_change_fk_address
        FOREIGN KEY (address)
            REFERENCES ftd_account (address)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS ftd_identity_change_idx_block_hash
    ON ftd_identity_change (block_hash);
CREATE INDEX IF NOT EXISTS ftd_identity_change_idx_block_number
    ON ftd_identity_change (block_number);
CREATE INDEX IF NOT EXISTS ftd_identity_change_idx_address
    ON ftd_identity_change (address);
CREATE INDEX IF NOT EXISTS ftd_identity_change_idx_address_block_number
    ON ftd_identity_change (address, block_number);
CREATE INDEX IF NOT EXISTS ftd_identity_change_idx_super_address
    ON ftd_identity_change (super_address);
CREATE INDEX IF NOT EXISTS ftd_identity_change_idx_display_sub_display
    ON ftd_identity_change (display, sub_display);