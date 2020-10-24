CREATE TABLE schemas (
    id serial NOT NULL PRIMARY KEY,
    version INT NOT NULL,
    subject VARCHAR NOT NULL,
    format VARCHAR NOT NULL,
    definition TEXT NOT NULL
)
