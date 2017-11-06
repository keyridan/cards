use std::io;
use hyper::Error;
use translation::{translator, request, response, language};
use serde_json;
use std::env;

pub fn request_translation(word_to_translate: request::Translation) -> Result<response::Translation, Error> {
    translator::request_translate(&translation_url(), word_to_translate)
}

fn translation_url() -> String {
    format!("{}/translate", env::var("TRANSLATION_URL").expect("TRANSLATION_URL must be set"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let (translation_request, expected_translation) = en_de_translation();
        let result = request_translation(translation_request).unwrap();
        println!("{:?}", result);
        assert_eq!(result, expected_translation);
    }

    fn en_de_translation() -> (request::Translation, response::Translation) {
        let word = "exam";
        let from_language = language::Language::EN;
        let to_language = language::Language::DE;
        let translation_request = request::Translation::new(word)
            .from(from_language)
            .to(to_language);
        let expected_translation = response::Translation::new(word, "Prüfung", from_language)
            .with_words(de_expected_translated_words());
        (translation_request, expected_translation)
    }

    fn de_expected_translated_words() -> Vec<response::TranslatedWords> {
        vec![
            response::TranslatedWords::new("Prüfung", Some("die".to_string())),
            response::TranslatedWords::new("Examen", Some("das".to_string())),
            response::TranslatedWords::new("Klausur", Some("die".to_string())),
        ]
    }
}