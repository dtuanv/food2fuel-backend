CREATE TABLE Item (
    id BIGSERIAL PRIMARY KEY,
    description VARCHAR(512) NOT NULL,
    category VARCHAR(255),
    date VARCHAR(255)
);

/*
id BIGSERIAL PRIMARY KEY,

 Mit BIGSERIAL wird eine Spalte vom Typ BIGINT erstellt, die automatisch inkrementiert wird.
 */

