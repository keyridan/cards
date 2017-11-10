-- Your SQL goes here
CREATE TABLE words (
  id SERIAL PRIMARY KEY,
  language varchar(10) REFERENCES languages(code) NOT NULL,
  sex varchar(2) REFERENCES sex(code),
  text VARCHAR NOT NULL,
  UNIQUE(text, language)
);