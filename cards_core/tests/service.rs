use connection::*;
use cards_core::service::*;
use cards_core::repository;
use cards_core::models::*;

#[test]
fn test_create_translation() {
    // given
    let connection = establish();
    let test_word_from = test_new_word();
    let test_words_to = test_new_words_to();

    // when
    let (word_from, words_to) = create_translation(&connection, test_word_from, test_words_to).unwrap();

    // then
    let result_words = repository::find_by_word(&connection, &word_from).to_word();
    assert_eq!(result_words, words_to);
}

#[test]
fn when_word_not_exist_then_not_found() {
    // given
    let connection = establish();
    let test_word = test_new_word();

    // when
    let result_word = find_translation(&connection, &test_word.text, &test_word.language, "TEST");

    // then
    assert_eq!(result_word, None);
}

#[test]
fn when_word_with_same_text_and_language_exist_and_translation_not_exist_then_word_found() {
    // given
    let connection = establish();
    let expected_word = saved_word_from(&connection);
    let not_found_translations = Vec::<Word>::new();

    // when
    let result_word = find_translation(&connection, &expected_word.text, &expected_word.language, "TEST");

    // then
    assert_eq!(result_word, Some((expected_word, not_found_translations)));
}

#[test]
fn when_word_with_same_text_and_language_exist_and_translation_exist_then_found() {
    // given
    let connection = establish();
    let test_word_from = test_new_word();
    let test_words_to = test_new_words_to();
    let (word_from, words_to) = create_translation(&connection, test_word_from, test_words_to).unwrap();
    let found_translation = vec![words_to[0].clone()];

    // when
    let result_word = find_translation(&connection, &word_from.text, &word_from.language, "DE");

    // then
    assert_eq!(result_word, Some((word_from, found_translation)));
}

#[test]
fn when_word_with_same_text_and_another_language_exist_and_translation_not_then_word_found() {
    // given
    let connection = establish();
    let test_word = saved_word_from(&connection);
    let not_found_translations = Vec::<Word>::new();

    // when
    let result_word = find_translation(&connection, &test_word.text, "ANOTHER_LANGUAGE", "TEST");

    // then
    assert_eq!(result_word, Some((test_word, not_found_translations)));
}

#[test]
fn when_word_with_same_text_and_another_language_and_translation_exist_then_found() {
    // given
    let connection = establish();
    let test_word_from = test_new_word();
    let test_words_to = test_new_words_to();
    let (word_from, words_to) = create_translation(&connection, test_word_from, test_words_to).unwrap();
    let found_translation = vec![words_to[0].clone()];

    // when
    let result_word = find_translation(&connection, &word_from.text, "ANOTHER_LANGUAGE", "DE");

    // then
    assert_eq!(result_word, Some((word_from, found_translation)));
}