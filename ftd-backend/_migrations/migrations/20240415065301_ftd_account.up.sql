CREATE TABLE IF NOT EXISTS ftd_account
(
    address                          VARCHAR(64) PRIMARY KEY,
    created_at                       TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at                       TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);