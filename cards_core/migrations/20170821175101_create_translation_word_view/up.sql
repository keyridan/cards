-- Your SQL goes here
CREATE VIEW translation_words AS
SELECT w.id word_id, translatedWord.* from words w
  JOIN translations tr on tr.word_from = w.id
  JOIN words translatedWord on translatedWord.id = tr.word_to;