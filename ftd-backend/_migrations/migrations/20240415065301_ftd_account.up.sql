CREATE TABLE IF NOT EXISTS ftd_account
(
    address                          VARCHAR(64) PRIMARY KEY,
    display                          VARCHAR(256),
    legal                            VARCHAR(256),
    web                              VARCHAR(256),
    riot                             VARCHAR(128),
    email                            VARCHAR(128),
    twitter                          VARCHAR(128),
    judgement                        VARCHAR(64),
    super_address                    VARCHAR(64),
    sub_display                      VARCHAR(256),
    created_at                       TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at                       TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT ftd_account_fk_super_account
        FOREIGN KEY (super_address)
            REFERENCES ftd_account (address)
            ON DELETE RESTRICT
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS ftd_account_idx_display
    ON ftd_account (display);
CREATE INDEX IF NOT EXISTS ftd_account_idx_sub_display
    ON ftd_account (display);
CREATE INDEX IF NOT EXISTS ftd_account_idx_display_sub_display
    ON ftd_account (display, sub_display);