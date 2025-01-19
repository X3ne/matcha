CREATE TABLE "user"
(
    id               BIGINT PRIMARY KEY NOT NULL UNIQUE,
    email            VARCHAR(255)       NOT NULL UNIQUE,
    username         VARCHAR(255)       NOT NULL UNIQUE,
    last_name        VARCHAR(255)       NOT NULL,
    first_name       VARCHAR(255)       NOT NULL,
    password         VARCHAR(255),
    is_active        BOOLEAN            NOT NULL DEFAULT FALSE,
    activation_token VARCHAR(255)       NOT NULL,
    created_at       TIMESTAMP          NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMP          NOT NULL DEFAULT NOW()
);

SELECT manage_updated_at('user');
