-- Your SQL goes here
CREATE TABLE plugins (
    id TEXT PRIMARY KEY,
    search_url TEXT NOT NULL,
    search TEXT NOT NULL,
    chapters_url TEXT NOT NULL,
    get_chapters TEXT NOT NULL,
    pages_url TEXT NOT NULL,
    get_pages TEXT NOT NULL
);