use std::collections::HashMap;

use reqwest;
use rand;
use rand::seq::SliceRandom;

static WORDS_URL: &str = "http://java.lib-ido.de/Webapp/words.jsp";

#[derive(Debug)]
pub struct WordList {
    pub pre: Vec<String>,
    pub post: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
    DownloadFailed(String),
    JsonError(String),
}

impl WordList {
    fn from_map(map: HashMap<String, Vec<String>>) -> WordList {
        let pre = map.get("pre").unwrap().clone();
        let post = map.get("post").unwrap().clone();
        WordList {
            pre,
            post,
        }
    }
}

pub fn download_word_list() -> Result<WordList, Error> {
    let mut request = http_request(WORDS_URL);
    match &mut request {
        Err(error) => Err(Error::DownloadFailed(error.clone())),
        Ok(request) => {
            let response: reqwest::Result<HashMap<String, Vec<String>>> = request.json();
            match response {
                Err(error) => Err(Error::JsonError(error.to_string())),
                Ok(map) => Ok(WordList::from_map(map)),
            }
        }
    }
}

pub fn combination(words: &WordList) -> Option<String> {
    let mut rng = rand::thread_rng();
    let pre = words.pre.choose(&mut rng);
    let post = words.post.choose(&mut rng);
    if pre.is_none() || post.is_none() {
        return None;
    }
    Some(combine(pre.unwrap(), post.unwrap()))
}

fn http_request(url: &str) -> Result<reqwest::Response, String> {
    let response = reqwest::get(url);
    match response {
        Ok(response) => Ok(response),
        Err(error) => Err(error.to_string()),
    }
}

fn combine(pre: &str, post: &str) -> String {
    format!("{}{}", pre, post.to_lowercase())
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn combine_with_uppercase() {
        let combined = combine("Hoden", "Granate");
        assert_eq!("Hodengranate", combined.to_str());
    }
}