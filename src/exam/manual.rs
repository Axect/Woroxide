#[derive(Debug, Clone)]
pub enum Candidate {
    One(String),
    Two(String),
    Three(String),
    Four(String),
}

#[derive(Debug, Clone)]
pub struct ManualTest {
    word: String,
    answer: String,
    candi: (String, String, String)
}

impl ManualTest {
    fn from_strs(w: &str, a: &str, c: (&str, &str, &str)) -> Self {
        Self {
            word: w.to_string(),
            answer: a.to_string(),
            candi: (c.0.to_string(), c.1.to_string(), c.2.to_string())
        }
    }
}