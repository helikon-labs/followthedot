CREATE TABLE IF NOT EXISTS ftd_transfer_volume
(
    from_address VARCHAR(64)                 NOT NULL,
    to_address   VARCHAR(64)                 NOT NULL,
    volume       VARCHAR(128)                NOT NULL DEFAULT 0,
    count        INTEGER                     NOT NULL DEFAULT 1,
    created_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    PRIMARY KEY (from_address, to_address),
    CONSTRAINT ftd_transfer_volume_fk_from_address
        FOREIGN KEY (from_address)
            REFERENCES ftd_account (address)
            ON DELETE RESTRICT
            ON UPDATE CASCADE,
    CONSTRAINT ftd_transfer_volume_fk_to_address
        FOREIGN KEY (to_address)
            REFERENCES ftd_account (address)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS ftd_transfer_idx_from_address
    ON ftd_transfer_volume (from_address);
CREATE INDEX IF NOT EXISTS ftd_transfer_idx_to_address
    ON ftd_transfer_volume (to_address);