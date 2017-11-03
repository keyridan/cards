use std::io;
use hyper::Error;
use translation::{translator, request, response, language};
use serde_json;

pub fn request_translation(word_to_translate: request::Translation) -> Result<response::Translation, Error> {
    translator::request_translate("http://localhost:8000/translate", word_to_translate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let result = request_translation(translation_request()).unwrap();
        println!("{:?}", result);
        assert_eq!(result, expected_translation());
    }

    fn translation_request() -> request::Translation {
        request::Translation::new("Prüfung")
            .from(language::Language::DE)
            .to(language::Language::EN)
    }

    fn expected_translation() -> response::Translation {
        response::Translation::new("Prüfung", "exam", language::Language::DE)
    }
}