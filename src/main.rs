

// TODO redefine the struct to get the actual synonyms
// TODO implement loop for multiple queries
// TODO figure out non-async functions

//* https://app.quicktype.io/ for strongly typed json data



use dotenv;

use std::env;
use std::string::String;
use std::process;

use tscaurus::Config;




// TODO main function non-async
#[tokio::main]
async fn main() {

    let key = &dotenv::var("mw_api_key").unwrap();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(| err |  {
            println!("Not enough arguments: {}", err);
            process::exit(1);
    });
    
    println!("\n\nTesting mode? {}\n", config.testing);
    
    if !config.testing {
        println!("Command: {}", config.command);
        println!("Query: {}\n", config.query);
    }
    
    let client = reqwest::Client::new();

    if config.testing {
        tscaurus::run_tests(&client, key).await;
    } else {
        if config.command == "get" {
            let synonyms = tscaurus::thesaurus_request(&client, config.query.as_str(), key.as_str())
                .await;
            println!("{:?}", synonyms);
        }
    }

}






