CREATE TABLE IF NOT EXISTS ftd_identity_transfer_updater_state
(
    id            INTEGER PRIMARY KEY,
    block_hash    VARCHAR(64)                 NOT NULL,
    block_number  BIGINT                      NOT NULL,
    is_successful BOOLEAN                     NOT NULL,
    error_log     TEXT,
    updated_at    TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

INSERT INTO ftd_identity_transfer_updater_state(id, block_hash, block_number, is_successful, error_log)
VALUES (1, '', 0, false, NULL);