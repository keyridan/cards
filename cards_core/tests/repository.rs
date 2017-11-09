use connection::*;
use cards_core::repository::*;
use cards_core::models::{ToWordConvertible, Word, NewWord};

#[test]
fn when_save_new_word_then_saved_word() {
    // given
    let connection = establish();
    let test_word = test_new_word();

    // when
    let result_word = save(&connection, &test_word).unwrap();

    // then
    println!("{}", result_word.text);
    assert_eq!(result_word.text, "экзамен");
}

#[test]
fn when_save_same_word_then_update() {
    // given
    let connection = establish();
    let test_word = test_new_word();
    let result_word = save(&connection, &test_word).unwrap();

    let same_word_with_updated_field = NewWord {
        sex: Some("N".to_string()),
        ..test_word
    };

    // when
    let result_word = save(&connection, &same_word_with_updated_field).unwrap();

    // then
    println!("{:?}", result_word);
    assert_eq!(result_word.text, "экзамен");
    assert_eq!(result_word.language, "RU");
    assert_eq!(result_word.sex, Some("N".to_string()));
}

#[test]
fn when_save_word_with_another_language_but_the_same_text_then_save() {
    // given
    let connection = establish();
    let test_word = test_new_word();
    save(&connection, &test_word).unwrap();

    let same_word_with_updated_field = NewWord {
        language: "EN",
        ..test_word
    };

    // when
    save(&connection, &same_word_with_updated_field).unwrap();

    // then
    let found_words = find_all_by_text(&connection, "экзамен");
    assert_eq!(found_words.len(), 2);
    assert_eq!(found_words[0].text, "экзамен");
    assert_eq!(found_words[1].text, "экзамен");
    assert_eq!(found_words[0].language, "RU");
    assert_eq!(found_words[1].language, "EN");
}

#[test]
fn when_word_exist_then_ok_saved_word() {
    // given
    let connection = establish();
    let test_word = test_new_word();
    let saved_word = save(&connection, &test_word).unwrap();

    // when
    let found_word = find_by_text(&connection, "экзамен");

    // then
    assert_eq!(found_word, Some(saved_word));
}

#[test]
fn when_word_not_exist_then_error() {
    // given
    let connection = establish();

    // when
    let found_word = find_by_text(&connection, "экзамен");

    // then
    assert_eq!(found_word, None);
}

#[test]
fn test_save_translation() {
    // given
    let connection = establish();
    let saved_word_from = saved_word_from(&connection);
    let saved_words_to = saved_words_to(&connection);

    // when
    translate_all(&connection, &saved_word_from, &saved_words_to);

    // then
    let result_words = find_by_word(&connection, &saved_word_from).to_word();
    assert_eq!(result_words, saved_words_to);
}

#[test]
fn when_translate_same_word_then_update() {
    // given
    let connection = establish();
    let saved_word_from = saved_word_from(&connection);
    let saved_words_to = saved_words_to(&connection);
    translate_all(&connection, &saved_word_from, &saved_words_to);
    let same_word_to = &saved_words_to[1];

    // when
    translate(&connection, &saved_word_from, same_word_to);

    // then
    let result_words = find_by_word(&connection, &saved_word_from).to_word();
    println!("{:?}", result_words);
    assert_eq!(result_words, saved_words_to);
}

#[test]
fn when_translate_another_word_to_then_2_translations() {
    // given
    let connection = establish();
    let saved_word_from = saved_word_from(&connection);
    let saved_word_to = save_test_word(&connection, Box::new(german_word));
    translate(&connection, &saved_word_from, &saved_word_to);
    let another_word_to = save_test_word(&connection, Box::new(english_word));

    // when
    translate(&connection, &saved_word_from, &another_word_to);

    // then
    let result_words = find_by_word(&connection, &saved_word_from).to_word();
    println!("{:?}", result_words);
    assert_eq!(result_words, vec!(saved_word_to, another_word_to));
}

#[test]
fn when_saved_word_but_not_saved_translations_then_empty() {
    // given
    let connection = establish();
    let saved_word_from = saved_word_from(&connection);

    // when
    let translations = find_by_word(&connection, &saved_word_from).to_word();

    // then
    assert!(translations.is_empty());
}