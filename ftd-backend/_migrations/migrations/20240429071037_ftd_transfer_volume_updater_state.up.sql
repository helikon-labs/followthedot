CREATE TABLE IF NOT EXISTS ftd_transfer_volume_updater_state
(
    id                         INTEGER PRIMARY KEY,
    last_processed_transfer_id INTEGER                     NOT NULL,
    created_at                 TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now(),
    updated_at                 TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT now()
);

INSERT INTO ftd_transfer_volume_updater_state(id, last_processed_transfer_id) VALUES(1, 0);