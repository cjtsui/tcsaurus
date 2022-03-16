use reqwest;
use serde::{Deserialize, Serialize};
use dotenv;

use std::string::String;
// //! redefine the struct to get the actual synonyms


//* https://app.quicktype.io/



// TODO main function non-async
#[tokio::main]
async fn main() {
    let word = "conduct";
    let key = dotenv::var("mw_api_key").unwrap();
    let syns = thesaurus_request(word, key.as_str()).await;
    println!("{:?}", syns)
}



// TODO function needs to return a result type, list of strings or error
async fn thesaurus_request(word: &str, key: &str) -> Vec<WelcomeElement> {

    let url = format!(
        "https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{word}?key={key}",
        word = word,
        key = key,
    );
        
    let client = reqwest::Client::new();
    let request = client
        .get(url)
        .send()
        .await
        .unwrap();

    match request.status() {
        reqwest::StatusCode::OK => {
            match request.json::<ThesaurusHeader>().await {
                Ok(parsed) => {
                    return parsed;
                },
                Err(serde_err) => {
                    panic!("The request didn't match the shape expected: {:?}", serde_err);
                },
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Token is invalid");
        }
        other => {
            panic!("Error: {:?}", other);
        }
    };

}

pub type ThesaurusHeader = Vec<WelcomeElement>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WelcomeElement {
    // meta: Option<Meta>,
    fl: Option<Fl>,
    def: Option<Vec<Def>>,
    // shortdef: Option<Vec<String>>,
    // vrs: Option<Vec<Vr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Def {
    sseq: Option<Vec<Vec<Vec<SseqElement>>>>,
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SseqElement {
    Enum(SseqEnum),
    SseqClass(SseqClass),
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
    syn_list: Option<Vec<Vec<Word>>>,
    rel_list: Option<Vec<Vec<Word>>>,
    near_list: Option<Vec<Vec<Word>>>,
    // ant_list: Option<Vec<Vec<List>>>,
    // phrase_list: Option<Vec<Vec<PhraseList>>>,
    // ins: Option<Vec<In>>,
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
    t: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Fl {
    #[serde(rename = "adjective")]
    Adjective,
    #[serde(rename = "adverb")]
    Adverb,
    #[serde(rename = "noun")]
    Noun,
    #[serde(rename = "verb")]
    Verb,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Section {
    #[serde(rename = "alpha")]
    Alpha,
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
