CREATE TABLE schemas (
    id serial NOT NULL,
    version INT NOT NULL,
    subject VARCHAR NOT NULL,
    format VARCHAR NOT NULL,
    definition TEXT NOT NULL,
    PRIMARY KEY(id, version)
)
