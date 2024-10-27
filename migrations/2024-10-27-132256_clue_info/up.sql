-- Your SQL goes here
CREATE TABLE clue_info(
    clue_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    channel_id BIGINT NOT NULL,
    clue_type TEXT NOT NULL,
    clue_body TEXT NOT NULL,
    FOREIGN KEY(channel_id) REFERENCES game(channel_id)
)