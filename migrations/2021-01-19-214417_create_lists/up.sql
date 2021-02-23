CREATE TABLE lists (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    title varchar(256) NOT NULL,
    subtitle varchar(256) NOT NULL
);