use serde_json::{from_str, Value};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    let word = args.get(1);
    match word {
        Some(x) => {
            let collection = collection(&x);
            let mut file_data = get_data(collection);
            let definitions = file_data[x].take();
            if definitions.is_null() {
                println!("{} is not in our dictionary", x);
                process::exit(1);
            }

            if let Some(definitions_array) = definitions.as_array() {
                print(definitions_array, &definitions);
            }
        }
        None => {
            println!("Please specify the word");
            process::exit(1)
        }
    }
}

fn print(v: &Vec<Value>, raw: &Value) {
    for i in v {
        let part_of_speech = &i["part_of_speech"];
        let category = &i["category"];
        let explanation = &i["explanation"];
        if part_of_speech.is_null() || category.is_null() || explanation.is_null() {
            println!("Failed to display defination, here is the raw data:\n{}", raw.to_string())
        }
        println!("{} : {}  -> {}\n\n", part_of_speech, category, explanation);
    }
}

fn get_data(collection: String) -> Value {
    let file_path = format!("data/{}.json", collection);
    match fs::read_to_string(file_path) {
        Ok(x) => return parse(&x, collection),
        Err(_) => {
            println!(
                "data/{}.json not found, maybe the dictionary was deleted",
                collection
            );
            process::exit(1);
        }
    }
}

fn parse(json_string: &str, collection: String) -> Value {
    match from_str(json_string) {
        Ok(y) => return y,
        Err(_) => {
            println!(
                "data/{}.json failed to parse, maybe someone tampured the dictionary files",
                collection
            );
            process::exit(1);
        }
    }
}

fn collection(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let c = chars.get(0);
    match c {
        Some(x) => {
            return x.to_string();
        }
        None => {
            println!("Word must have atleast 1 character");
            process::exit(1)
        }
    }
}
