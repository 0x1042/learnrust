CREATE TABLE user_info (
    id SERIAL PRIMARY KEY,
    nick_name VARCHAR NOT NULL,
    email TEXT NOT NULL,
    invalid BOOLEAN NOT NULL DEFAULT FALSE
)