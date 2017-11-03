use super::models::*;
use diesel::*;
use connection::*;

pub fn save(connection: &DieselPgConnection, new_word: NewWord) -> Result<Word, DBError> {
    insert(&new_word)
        .into(words::table)
        .get_result::<Word>(connection)
}

pub fn save_all(connection: &DieselPgConnection, new_words: Vec<NewWord>) -> Result<Vec<Word>, DBError> {
    insert(&new_words)
        .into(words::table)
        .get_results::<Word>(connection)
}

pub fn translate(connection: &DieselPgConnection, word_from: &Word, word_to: &Word) -> Result<Vec<Translation>, DBError> {
    let translations = word_from.new_translation(word_to);
    insert(&translations)
        .into(translations::table)
        .get_results::<Translation>(connection)
}

pub fn translate_all(connection: &DieselPgConnection, word_from: &Word, words_to: &Vec<Word>) -> Result<Vec<Translation>, DBError> {
    let translations = word_from.new_translations(words_to);
    insert(&translations)
        .into(translations::table)
        .get_results::<Translation>(connection)
}

pub fn find_by_text(connection: &DieselPgConnection, word_text: &str) -> Option<Word> {
    words::table
        .filter(words::text.eq(word_text))
        .first::<Word>(connection)
        .optional()
        .expect("Database error")
}

pub fn find_by_word(connection: &DieselPgConnection, word_from: &Word) -> Vec<TranslatedWord> {
    TranslatedWord::belonging_to(word_from)
        .load::<TranslatedWord>(connection)
        .expect("Error loading translated_words")
}