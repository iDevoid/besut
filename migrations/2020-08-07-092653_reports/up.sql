-- Your SQL goes here
CREATE TABLE reports (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
)