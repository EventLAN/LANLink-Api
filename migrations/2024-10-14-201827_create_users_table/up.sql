-- Your SQL goes here
CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    publicId uuid NOT NULL,
    username VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,

    discord_username VARCHAR NOT NULL,
    steam_url VARCHAR NOT NULL
)