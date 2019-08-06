use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalWords {
    total: Vec<Words>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Words {
    chapter: usize,
    words: Vec<Word>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    word: String,
    mean: Vec<String>,
}

impl TotalWords {
    pub fn from_file(path: &str) -> Self {
        let json_path = Path::new(path);
        let json_file = File::open(json_path).expect("Can't open json file");
        let words_vec: Self = serde_json::from_reader(json_file).expect("Can't parse json");
        words_vec
    }

    pub fn get_words_vec(&self) -> Vec<Words> {
        self.total.clone()
    }

    pub fn get_specific_words(&self, chap: usize) -> Option<Words> {
        for i in 0 .. self.total.len() {
            let temp = &self.total[i];
            if temp.get_chapter() == chap {
                return Some(temp.clone())
            }
        }
        None
    }
}

impl Words {
    pub fn get_chapter(&self) -> usize {
        self.chapter
    }

    pub fn get_word_vec(&self) -> Vec<Word> {
        self.words.clone()
    }
}

impl Word {
    pub fn get_word(&self) -> String {
        self.word.clone()
    }

    pub fn get_mean(&self) -> Vec<String> {
        self.mean.clone()
    }

    pub fn match_with_word(&self, trial: String) -> bool {
        trial.trim() == self.word.trim()
    }

    pub fn match_with_mean(&self, trial: String) -> bool {
        self.mean.clone().into_iter()
            .any(|x| {
                x.trim() == trial.trim()
            })
    }
}

