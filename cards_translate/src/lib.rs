extern crate translation;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;

pub mod service;

use translation::{request, response, language};

struct TranslationRequest {
    pub text: String,
    pub language_from: language::Language,
    pub language_to: language::Language
}

struct TranslatedWord {
    pub text: String,
    pub language: language::Language,
    pub sex: Option<String>,
}
//
//fn translate(word: Word) -> Vec<TranslatedWord> {
//    service::request_translation()
//}