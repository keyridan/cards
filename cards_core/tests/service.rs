use connection::*;
use cards_core::service::*;
use cards_core::repository::*;
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
    let result_words = find_by_word(&connection, &word_from).to_word();
    assert_eq!(result_words, words_to);
}