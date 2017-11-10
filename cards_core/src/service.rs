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
        let word_from = save(connection, &new_word)?;
        let words_to = save_all(connection, translation_words)?;
        translate_all(connection, &word_from, &words_to);
        Ok((word_from, words_to))
    })
}

pub fn find_translation(connection: &DieselPgConnection, text: &str, language_from: &str, language_to: &str) -> Option<(Word, Vec<Word>)> {
    let find_translation_by_word = |word: Word| {
        let translations = find_translation_to_language_by_word(connection, &word, language_to).to_word();
        Some((word, translations))
    };
    let find_by_text = || {
        find_by_text(connection, text)
            .and_then(|word| find_translation_by_word(word))
    };
    find_by_text_and_language(connection, text, language_from)
        .map_or_else(find_by_text, |word| find_translation_by_word(word))
}
