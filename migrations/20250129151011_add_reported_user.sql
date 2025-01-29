CREATE TABLE user_report
(
    id          BIGINT PRIMARY KEY NOT NULL UNIQUE,
    reporter_id BIGINT             NOT NULL REFERENCES user_profile (id),
    reported_id BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    reason      TEXT,
    reported_at TIMESTAMP          NOT NULL DEFAULT NOW(),
    UNIQUE (reporter_id, reported_id)
);
