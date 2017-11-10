-- Your SQL goes here
CREATE TABLE translations (
  id SERIAL PRIMARY KEY,
  word_from SERIAL REFERENCES words(id) NOT NULL,
  word_to SERIAL REFERENCES words(id) NOT NULL,
  UNIQUE(word_from, word_to)
);


