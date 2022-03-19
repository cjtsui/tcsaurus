use reqwest::{Client};
use serde::{Deserialize, Serialize};

use std::string::String;
use std::process;

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

        thesaurus_request(client, word, key.as_str()).await
            .unwrap_or_else(| err | {
                println!("Something went wrong with the API request: {}", err);
                process::exit(1);
            });

        println!("Passed!\n");
        
    }

}



// TODO function needs to return a result type, list of strings or error
pub async fn thesaurus_request(client: &Client, word: &str, key: &str) -> Result<Vec<ThesaurusHeader>, String> {

    let url = format!(
        "https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{word}?key={key}",
        word = word,
        key = key,
    );
        
    let request = client
        .get(url)
        .send()
        .await
        .unwrap_or_else(| err | {
            println!("API error: {}", err);
            process::exit(1); // change later
        });

    match request.status() {
        reqwest::StatusCode::OK => {
            match request.json::<ThesaurusResponse>().await {
                Ok(parsed) => {
                    return Ok(parsed);
                },
                Err(serde_err) => {
                    return Err(format!("The request didn't match the shape expected: {:?}", serde_err));
                },
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            return Err("Token is invalid".to_string());
        }
        other => {
            return Err(format!("Error, status code: {:?}", other));
        }
    };

}


fn extract_definition(response: ThesaurusHeader) -> Synonym {

    let word_type = response.fl.clone();
    let defs = response.def;

    let syn_definitions = Vec::new();

    for def in defs.iter() {
        let sseq = &def.sseq;
    }

    return Synonym {
        word_type,
        definitions: syn_definitions,
    };

}

struct Synonym {
    word_type: Fl,
    definitions: Vec<Definition>,
}


struct Definition {
    index: u8,
    definition: String
}



pub type ThesaurusResponse = Vec<ThesaurusHeader>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThesaurusHeader {
    // meta: Option<Meta>,
    fl: Fl,
    def: Vec<Def>,
    // shortdef: Option<Vec<String>>,
    // vrs: Option<Vec<Vr>>,
}


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Fl {
    #[serde(rename = "adjective")]
    Adjective,
    #[serde(rename = "adverb")]
    Adverb,
    #[serde(rename = "noun")]
    Noun,
    #[serde(rename = "verb")]
    Verb,
    #[serde(rename = "interjection")]
    Interjection,
    #[serde(rename = "phrase")]
    Phrase,
    #[serde(rename = "plural noun")]
    PluralNoun,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Def {
    sseq: Vec<Vec<Vec<SseqElement>>>,
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SseqElement {
    Sense(SseqEnum),
    SseqClassEnum(SseqClass),
}


#[derive(Debug, Serialize, Deserialize)]
pub enum SseqEnum {
    #[serde(rename = "sense")]
    Sense,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SseqClass {
    sn: Option<String>,
    dt: Option<Vec<Vec<DtUnion>>>, // //! something wrong here
    // syn_list: Option<Vec<Vec<Word>>>,
    // rel_list: Option<Vec<Vec<Word>>>,
    // near_list: Option<Vec<Vec<Word>>>,
    // ant_list: Option<Vec<Vec<List>>>,
    // phrase_list: Option<Vec<Vec<PhraseList>>>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DtUnion {
    DtClassArray(Vec<DtClass>),
    String(String),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    wd: Option<String>,
    // wsls: Option<Vec<String>>,
    // wvrs: Option<Vec<Wvr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DtClass {
    // t: Option<String>,
}



/*
#[derive(Serialize, Deserialize, Debug)]
struct CardInformation {
    balance: i32,
    description: String,
    id: u32,
    pubkey: String
}

let cards = self.client.get(format!("http://localhost:4000/api/users/{}/cards", &self.credentials.username))
    .header("Accept", "application/json")
    .basic_auth(&self.credentials.username, Some(&self.credentials.password))
    .send()?
    .json::<Vec<CardInformation>>()?;

println!("{:#?}", cards);


*/


/*
use serde::{Deserialize, Serialize};

pub type Welcome = Vec<WelcomeElement>;

#[derive(Serialize, Deserialize)]
pub struct WelcomeElement {
    meta: Meta,
    hwi: Hwi,
    fl: String,
    def: Vec<Def>,
    shortdef: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Def {
    sseq: Vec<Vec<Vec<SseqElement>>>,
}

#[derive(Serialize, Deserialize)]
pub struct SseqClass {
    sn: String,
    dt: Vec<Vec<DtUnion>>,
    syn_list: Vec<Vec<SynList>>,
    rel_list: Vec<Vec<RelList>>,
    phrase_list: Option<Vec<Vec<List>>>,
    near_list: Option<Vec<Vec<List>>>,
    ant_list: Option<Vec<Vec<List>>>,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    wd: String,
}

#[derive(Serialize, Deserialize)]
pub struct DtClass {
    t: String,
}

#[derive(Serialize, Deserialize)]
pub struct RelList {
    wd: String,
    wvrs: Option<Vec<Wvr>>,
}

#[derive(Serialize, Deserialize)]
pub struct Wvr {
    wvl: String,
    wva: String,
}

#[derive(Serialize, Deserialize)]
pub struct SynList {
    wd: String,
    wvrs: Option<Vec<Wvr>>,
    wsls: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Hwi {
    hw: String,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    id: String,
    uuid: String,
    src: String,
    section: String,
    target: Target,
    stems: Vec<String>,
    syns: Vec<Vec<String>>,
    ants: Vec<Vec<String>>,
    offensive: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Target {
    tuuid: String,
    tsrc: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SseqElement {
    SseqClass(SseqClass),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DtUnion {
    DtClassArray(Vec<DtClass>),
    String(String),
}

*/




