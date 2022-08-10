CREATE TABLE users (
    id uuid NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    surname VARCHAR NOT NULL,
    access_code INT NOT NULL,
    accounting_day INT NOT NULL
);