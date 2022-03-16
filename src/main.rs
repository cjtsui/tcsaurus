use reqwest;
use serde::{Deserialize, Serialize};


// //! redefine the struct to get the actual synonyms

// TODO main function non-async
#[tokio::main]
async fn main() {
    let word = "conduct";
    let key = "eed63e22-8dc4-4ae7-8b7b-d9afc1816118";
    let syns = thesaurus_request(word, key).await;
    // println!("{:?}", syns)
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
    meta: Option<Meta>,
    hwi: Option<Hwi>,
    fl: Option<Fl>,
    def: Option<Vec<Def>>,
    shortdef: Option<Vec<String>>,
    vrs: Option<Vec<Vr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Def {
    sseq: Option<Vec<Vec<Vec<SseqElement>>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SseqClass {
    sn: Option<String>,
    dt: Option<Vec<Vec<DtUnion>>>,
    syn_list: Option<Vec<Vec<List>>>,
    rel_list: Option<Vec<Vec<List>>>,
    near_list: Option<Vec<Vec<List>>>,
    ant_list: Option<Vec<Vec<List>>>,
    phrase_list: Option<Vec<Vec<PhraseList>>>,
    ins: Option<Vec<In>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    wd: Option<String>,
    wsls: Option<Vec<String>>,
    wvrs: Option<Vec<Wvr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wvr {
    wvl: Option<Vl>,
    wva: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DtClass {
    t: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct In {
    #[serde(rename = "if")]
    in_if: Option<String>,
    spl: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhraseList {
    wd: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hwi {
    hw: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    id: Option<String>,
    uuid: Option<String>,
    src: Option<Src>,
    section: Option<Section>,
    target: Option<Target>,
    stems: Option<Vec<String>>,
    syns: Option<Vec<Vec<String>>>,
    ants: Option<Vec<Vec<String>>>,
    offensive: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    tuuid: Option<String>,
    tsrc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vr {
    vl: Option<Vl>,
    va: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SseqElement {
    Enum(SseqEnum),
    SseqClass(SseqClass),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DtUnion {
    DtClassArray(Vec<DtClass>),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Vl {
    #[serde(rename = "also")]
    Also,
    #[serde(rename = "or")]
    Or,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SseqEnum {
    #[serde(rename = "sense")]
    Sense,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Fl {
    #[serde(rename = "adjective")]
    Adjective,
    #[serde(rename = "adverb")]
    Adverb,
    #[serde(rename = "noun")]
    Noun,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Section {
    #[serde(rename = "alpha")]
    Alpha,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Src {
    #[serde(rename = "coll_thes")]
    CollThes,
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

