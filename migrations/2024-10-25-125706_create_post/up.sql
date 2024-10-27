-- Your SQL goes here
CREATE TABLE games(
    channel_id BIGINT NOT NULL PRIMARY KEY,
    players TEXT NOT NULL,
    board TEXT NOT NULL,
    game TEXT NOT NULL
)