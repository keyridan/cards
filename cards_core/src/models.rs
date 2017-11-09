#[derive(Queryable)]
pub struct Languages {
    pub code: String
}

#[derive(Queryable)]
pub struct Sex {
    pub code: String
}

#[derive(Identifiable, Queryable, Associations, AsChangeset)]
#[derive(Clone, Debug, PartialEq)]
pub struct Word {
    pub id: i32,
    pub language: String,
    pub sex: Option<String>,
    pub text: String
}

impl Word {
    pub fn new_translation(&self, translation_word: &Word) -> Vec<NewTranslation> {
        let mut translations = Vec::<NewTranslation>::new();
        translations.push(NewTranslation::new(self, &translation_word));
        translations.push(NewTranslation::new(&translation_word, self));
        return translations;
    }

    pub fn new_translations(&self, translation_words: &Vec<Word>) -> Vec<NewTranslation> {
        translation_words.into_iter().fold(Vec::new(), |mut translations, translation_word| {
            translations.extend(self.new_translation(translation_word));
            return translations;
        })
    }
}

#[derive(Identifiable, Queryable, Associations, Debug)]
pub struct Translation {
    pub id: i32,
    pub word_from: i32,
    pub word_to: i32
}

#[derive(Identifiable, Clone, Queryable, Debug, Associations)]
#[table_name = "translation_words"]
#[primary_key(word_id, id)]
#[belongs_to(Word, foreign_key = "word_id")]
pub struct TranslatedWord {
    pub word_id: i32,
    pub id: i32,
    pub language: String,
    pub sex: Option<String>,
    pub text: String
}

impl Word {
    pub fn from(translated_word: &TranslatedWord) -> Word {
        Word {
            id: translated_word.id,
            language: translated_word.language.clone(),
            sex: translated_word.sex.clone(),
            text: translated_word.text.clone(),
        }
    }
}

impl ToWordConvertible<Word> for TranslatedWord {
    fn to_word(&self) -> Word {
        Word::from(self)
    }
}

pub trait ToWordConvertible<T> {
    fn to_word(&self) -> T;
}

impl<T> ToWordConvertible<Vec<Word>> for Vec<T> where T: ToWordConvertible<Word> {
    fn to_word(&self) -> Vec<Word> {
        let words: Vec<Word> = self.into_iter()
            .map(|translated_word| translated_word.to_word())
            .collect();
        words
    }
}

#[derive(Insertable, AsChangeset)]
#[table_name = "words"]
pub struct NewWord<'a> {
    pub text: &'a str,
    pub language: &'a str,
    pub sex: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "translations"]
pub struct NewTranslation {
    pub word_from: i32,
    pub word_to: i32,
}

impl NewTranslation {
    fn new(word_from: &Word, word_to: &Word) -> NewTranslation {
        NewTranslation {
            word_from: word_from.id,
            word_to: word_to.id,
        }
    }
}

table! {
    translation_words (word_id, id)  {
        word_id -> Integer,
        id -> Integer,
        language -> Text,
        sex -> Nullable<Text>,
        text -> Text,
    }
}

table! {
    words (id) {
        id -> Integer,
        language -> Text,
        sex -> Nullable<Text>,
        text -> Text,
    }
}

table! {
    translations (id) {
        id -> Integer,
        word_from -> Integer,
        word_to -> Integer,
    }
}
joinable!(translation_words -> words(id));
joinable!(translations -> words(word_from));