CREATE TABLE channel
(
    id            BIGINT PRIMARY KEY NOT NULL UNIQUE,
    name          VARCHAR(100)       NOT NULL,
    last_activity TIMESTAMP          NOT NULL DEFAULT NOW(),
    created_at    TIMESTAMP          NOT NULL DEFAULT NOW()
);

CREATE TABLE message
(
    id         BIGINT PRIMARY KEY NOT NULL UNIQUE,
    author_id  BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    channel_id BIGINT             NOT NULL REFERENCES channel (id) ON DELETE CASCADE,
    content    TEXT               NOT NULL,
    deleted    BOOLEAN            NOT NULL DEFAULT FALSE,
    sent_at    TIMESTAMP          NOT NULL,
    edited_at  TIMESTAMP                   DEFAULT NULL
);

CREATE TABLE channel_participant
(
    id         BIGINT PRIMARY KEY NOT NULL UNIQUE,
    profile_id BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    channel_id BIGINT             NOT NULL REFERENCES channel (id) ON DELETE CASCADE,
    joined_at  TIMESTAMP          NOT NULL DEFAULT NOW(),
    UNIQUE (profile_id, channel_id)
);
