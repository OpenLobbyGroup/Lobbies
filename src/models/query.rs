use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Query
{
    pub name: String,
}

impl Query
{
    pub fn get_tags(name: &String) -> Vec<String>
    {
        let mut words = Self::split_name(name);
        words.retain(|x| Self::valid_word(x));
        if words.len() == 1 && words[0].chars().count() > 7
        {
            words = Self::artificial_split(name);
        }
        words
    }
    pub fn split_name(name: &String) -> Vec<String>
    {
        name.split(|c: char| c == ' ' || c == '.' || c == '-')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_lowercase())
            .collect()
    }
    pub fn artificial_split(name: &String) -> Vec<String>
    {
        let mut res = Vec::new();
        name.chars().filter(|&x| x == ' ' || x == '.' || x == '-').for_each(|x|res.push(String::from(x)));
        res
    }
    pub fn valid_word(word: &str) -> bool
    {
        let count = word.chars().count();
        count >= 4 && count <= 9
    }
}
