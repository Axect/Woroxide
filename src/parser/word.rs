extern crate serde;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Words {
    words: Vec<Word>
}

impl Words {
    pub fn get_words(&self) -> Vec<Word> {
        self.words.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    word: String,
    mean: Vec<String>,
}
