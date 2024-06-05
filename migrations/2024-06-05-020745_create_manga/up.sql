-- Your SQL goes here
CREATE TABLE manga (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    img TEXT NOT NULL,
    extension TEXT NOT NULL,
    authors TEXT NOT NULL,
    artists TEXT NOT NULL,
    description TEXT NOT NULL
);
