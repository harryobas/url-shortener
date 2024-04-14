CREATE TABLE IF NOT EXISTS url_info (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT UNIQUE NOT NULL,
    long_url TEXT NOT NULL,
    short_url TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_key ON url_info(key);
