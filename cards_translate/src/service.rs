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