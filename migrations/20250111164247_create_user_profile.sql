-- this enable the postgis extension
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1
                       FROM pg_extension
                       WHERE extname = 'postgis') THEN
            CREATE EXTENSION postgis;
        END IF;
    END;
$$;


CREATE TYPE gender AS ENUM ('male', 'female');
CREATE TYPE sexual_orientation AS ENUM ('male', 'female', 'bisexual');

CREATE TABLE profile_tag
(
    id   BIGINT PRIMARY KEY NOT NULL UNIQUE,
    name VARCHAR(255)       NOT NULL UNIQUE
);

CREATE TABLE user_profile
(
    id                 BIGINT PRIMARY KEY    NOT NULL UNIQUE,
    user_id            BIGINT                NOT NULL UNIQUE REFERENCES "user" (id) ON DELETE CASCADE,
    name               VARCHAR(255)          NOT NULL,
    avatar_hash        VARCHAR(255),
    picture_hashes     VARCHAR(255)[]        NOT NULL DEFAULT '{}',
    bio                TEXT,
    age                INTEGER               NOT NULL
        CONSTRAINT user_profile_age_check CHECK (age >= 18),
    gender             GENDER                NOT NULL,
    sexual_orientation SEXUAL_ORIENTATION    NOT NULL,
    location           GEOMETRY(POINT, 4326) NOT NULL,
    rating             INTEGER               NOT NULL DEFAULT 0,
    created_at         TIMESTAMP             NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMP             NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_profile_location ON user_profile USING GIST (location);

SELECT manage_updated_at('user_profile');

CREATE TABLE join_user_profile_tag
(
    id              BIGINT PRIMARY KEY NOT NULL UNIQUE,
    user_profile_id BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    profile_tag_id  BIGINT             NOT NULL REFERENCES profile_tag (id) ON DELETE CASCADE,
    created_at      TIMESTAMP          NOT NULL DEFAULT NOW(),
    UNIQUE (user_profile_id, profile_tag_id)
);

CREATE INDEX idx_join_user_profile_tag_user_profile_id ON join_user_profile_tag (user_profile_id);

CREATE TABLE profile_view
(
    id                BIGINT PRIMARY KEY NOT NULL UNIQUE,
    user_profile_id   BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    viewer_profile_id BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    viewed_at         TIMESTAMP          NOT NULL DEFAULT NOW(),
    UNIQUE (user_profile_id, viewer_profile_id)
);

CREATE INDEX idx_profile_view_user_profile_id ON profile_view (user_profile_id);

CREATE TABLE profile_like
(
    id                    BIGINT PRIMARY KEY NOT NULL UNIQUE,
    user_profile_id       BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    liked_user_profile_id BIGINT             NOT NULL REFERENCES user_profile (id) ON DELETE CASCADE,
    liked_at              TIMESTAMP          NOT NULL DEFAULT NOW(),
    UNIQUE (user_profile_id, liked_user_profile_id)
);

CREATE INDEX idx_profile_like_user_profile_id ON profile_like (user_profile_id);
