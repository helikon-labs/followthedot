CREATE TABLE IF NOT EXISTS ftd_transfer
(
    id                    SERIAL PRIMARY KEY,
    block_hash            VARCHAR(64)                 NOT NULL,
    timestamp             BIGINT                      NOT NULL,
    extrinsic_index       INTEGER                     NOT NULL,
    extrinsic_event_index INTEGER                     NOT NULL,
    event_index           INTEGER                     NOT NULL,
    from_address          VARCHAR(64)                 NOT NULL,
    to_address            VARCHAR(64)                 NOT NULL,
    amount                VARCHAR(128)                NOT NULL,
    created_at            TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT ftd_transfer_fk_block_hash
        FOREIGN KEY (block_hash)
            REFERENCES ftd_block (hash)
            ON DELETE RESTRICT
            ON UPDATE CASCADE,
    CONSTRAINT ftd_transfer_fk_from_address
        FOREIGN KEY (from_address)
            REFERENCES ftd_account (address)
            ON DELETE RESTRICT
            ON UPDATE CASCADE,
    CONSTRAINT ftd_transfer_fk_to_address
        FOREIGN KEY (to_address)
            REFERENCES ftd_account (address)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS ftd_transfer_idx_from_address
    ON ftd_transfer (from_address);
CREATE INDEX IF NOT EXISTS ftd_transfer_idx_to_address
    ON ftd_transfer (to_address);
CREATE INDEX IF NOT EXISTS ftd_transfer_idx_from_address_to_address
    ON ftd_transfer (from_address, to_address);