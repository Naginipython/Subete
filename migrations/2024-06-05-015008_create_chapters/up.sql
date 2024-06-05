CREATE TABLE chapter (
    id TEXT PRIMARY KEY,
    manga_id TEXT NOT NULL,
    number FLOAT NOT NULL,
    title TEXT NOT NULL,
    page INTEGER NOT NULL,
    completed BOOLEAN NOT NULL,
    FOREIGN KEY(manga_id) REFERENCES manga(id)
);
