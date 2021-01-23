CREATE TABLE items (
    id BIGSERIAL PRIMARY KEY,
    list_id BIGINT NOT NULL,
    title varchar(256) NOT NULL,
    amount INTEGER NOT NULL
);