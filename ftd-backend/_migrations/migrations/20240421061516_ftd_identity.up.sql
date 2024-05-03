CREATE TABLE IF NOT EXISTS ftd_identity
(
    address      VARCHAR(64) PRIMARY KEY,
    display      VARCHAR(256),
    legal        VARCHAR(256),
    web          VARCHAR(256),
    riot         VARCHAR(128),
    email        VARCHAR(128),
    twitter      VARCHAR(128),
    is_confirmed BOOLEAN                     NOT NULL DEFAULT FALSE,
    is_invalid   BOOLEAN                     NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at   TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT ftd_identity_fk_address
        FOREIGN KEY (address)
            REFERENCES ftd_account (address)
            ON DELETE CASCADE
            ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS ftd_identity_idx_display
    ON ftd_identity (display);
CREATE INDEX IF NOT EXISTS ftd_identity_idx_legal
    ON ftd_identity (legal);
CREATE INDEX IF NOT EXISTS ftd_identity_idx_web
    ON ftd_identity (web);
CREATE INDEX IF NOT EXISTS ftd_identity_idx_riot
    ON ftd_identity (riot);
CREATE INDEX IF NOT EXISTS ftd_identity_idx_email
    ON ftd_identity (email);
CREATE INDEX IF NOT EXISTS ftd_identity_idx_twitter
    ON ftd_identity (twitter);
CREATE INDEX IF NOT EXISTS ftd_identity_idx_all
    ON ftd_identity (display, legal, web, riot, email, twitter);