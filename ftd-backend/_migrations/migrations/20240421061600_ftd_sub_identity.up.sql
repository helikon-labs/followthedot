CREATE TABLE IF NOT EXISTS ftd_sub_identity
(
    address       VARCHAR(64) PRIMARY KEY,
    super_address VARCHAR(64)                 NOT NULL,
    sub_display   VARCHAR(256),
    created_at    TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at    TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT ftd_sub_identity_fk_address
        FOREIGN KEY (address)
            REFERENCES ftd_account (address)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
    CONSTRAINT ftd_identity_fk_super_address
        FOREIGN KEY (super_address)
            REFERENCES ftd_account (address)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS ftd_sub_identity_idx_super_address
    ON ftd_sub_identity (super_address);
CREATE INDEX IF NOT EXISTS ftd_sub_identity_idx_sub_display
    ON ftd_sub_identity (sub_display);
