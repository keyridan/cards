extern crate translation;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;

pub mod service;

use translation::{request, response, language};

struct TranslatedWord {
    pub text: String,
    pub language: language::Language,
    pub sex: Option<String>,
}
//
//fn translate(word: request::Translation) -> Vec<TranslatedWord> {
//    let response = service::request_translation(word);
//    match response {
//        Ok(translation) => vec!(translation),
//        _ => Vec::new()
//    }
//}