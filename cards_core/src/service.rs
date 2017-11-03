use super::repository::*;
use super::models::*;
use super::connection::*;
use diesel::connection::*;

pub fn create_word_translation(new_word: NewWord, translation_words: Vec<NewWord>) -> Result<(Word, Vec<Word>), DBError> {
    let connection = establish();
    create_translation(&connection, new_word, translation_words)
}

pub fn create_translation(connection: &DieselPgConnection, new_word: NewWord, translation_words: Vec<NewWord>) -> Result<(Word, Vec<Word>), DBError> {
    connection.transaction(|| {
        let word_from = save(connection, new_word)?;
        let words_to = save_all(connection, translation_words)?;
        translate_all(connection, &word_from, &words_to);
        Ok((word_from, words_to))
    })
}
