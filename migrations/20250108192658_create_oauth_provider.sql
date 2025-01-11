CREATE TABLE oauth_provider
(
    id     BIGINT PRIMARY KEY NOT NULL UNIQUE,
    name   TEXT               NOT NULL UNIQUE,
    active BOOLEAN            NOT NULL DEFAULT TRUE
);


CREATE TABLE oauth_account
(
    id               BIGINT    NOT NULL PRIMARY KEY UNIQUE,
    user_id          BIGINT    NOT NULL REFERENCES "user" (id) ON DELETE CASCADE,
    provider_id      BIGINT    NOT NULL REFERENCES oauth_provider (id) ON DELETE CASCADE,
    provider_user_id TEXT      NOT NULL,
    access_token     TEXT,
    refresh_token    TEXT,
    expires_at       TIMESTAMP,
    created_at       TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (provider_id, provider_user_id),
    UNIQUE (user_id, provider_id)
);

SELECT manage_updated_at('oauth_account');
