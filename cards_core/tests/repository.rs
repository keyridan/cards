use connection::*;
use cards_core::repository::*;
use cards_core::models::ToWordConvertible;

#[test]
fn when_save_new_word_then_saved_word() {
    // given
    let connection = establish();
    let test_word = test_new_word();

    // when
    let expected_word = save(&connection, test_word).unwrap();

    // then
    println!("{}", expected_word.text);
    assert_eq!(expected_word.text, "экзамен");
}

#[test]
fn when_word_exist_then_ok_saved_word() {
    // given
    let connection = establish();
    let test_word = test_new_word();
    let saved_word = save(&connection, test_word).unwrap();

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