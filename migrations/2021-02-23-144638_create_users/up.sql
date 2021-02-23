CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    nick varchar(256) NOT NULL,
    email varchar(256) NOT NULL
);
