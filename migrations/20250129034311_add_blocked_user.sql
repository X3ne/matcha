CREATE TABLE user_block
(
    id         BIGINT PRIMARY KEY NOT NULL UNIQUE,
    blocker_id BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    blocked_id BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    blocked_at TIMESTAMP          NOT NULL DEFAULT NOW(),
    UNIQUE (blocker_id, blocked_id)
);
