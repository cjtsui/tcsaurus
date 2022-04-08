use reqwest::{Client};
use serde::{Deserialize, Serialize};

use std::string::String;
use std::process;

use serde_json::Value as JsonValue;
use jsonpath_rust::JsonPathQuery;



pub struct Config {
    pub testing: bool, // if true then run tests, if false then normal
    pub command: String,
    pub query: String, // default value is "Currently Running Tests"
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let length = args.len();
        if length == 1 {
            return Err("No arguments given");
        }
        let testing = args[1] == "test";
        if testing {
            return Ok (Config {
                testing: true,
                command: String::from("None"),
                query: String::from("Currently Running Tests"),
            });
        } else {
            if length < 3 {
                return Err("Only 1 argument given, which was not 'test'")
            }
            return Ok (Config {
                testing: false,
                command: args[1].clone(),
                query: args[2].clone(),
            });
        }
    }
}



pub async fn run_tests(client: &Client, key: &String) {

    let test_words = vec!("good", "cargo", "conduct", "he", "the", "happy", "merry", "example", "establishment");

    for word in test_words.iter() {
        println!("Testing {}...", word);
        thesaurus_request(client, word, key.as_str()).await;
        println!("Passed!\n");
    }

}



pub async fn thesaurus_request(client: &Client, word: &str, key: &str)  {

    let url = format!(
        "https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{word}?key={key}",
        word = word,
        key = key,
    );
        
    let response: JsonValue = client
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let _definition_path = "$[*].def[*].sseq[*][0][1].dt[0][1]";
    let syn_path = "$[*].def[*].sseq[*][0][1].syn_list[*][*].wd";

    if let JsonValue::Array(vals) = response.path(syn_path).unwrap() {
        let words: Vec<&str> =
            vals.iter()
                .map(| s | s.as_str().unwrap())
                .collect();
        println!("{:?}", words);
    }
    
}





