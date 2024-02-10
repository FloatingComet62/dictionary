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
                let result = filter(definitions_array, &definitions);
                table_print(&result);
            }
        }
        None => {
            end("Please specify the word");
        }
    }
}

fn end(msg: &str) {
    println!("{}", msg);
    process::exit(1);
}

pub struct Defination {
    pub part_of_speech: String,
    pub category: String,
    pub explanation: String,
}

fn filter(v: &Vec<Value>, raw: &Value) -> Vec<Defination> {
    let mut filtered = vec![];
    for i in v {
        let part_of_speech = &i["part_of_speech"];
        let category = &i["category"];
        let explanation = &i["explanation"];
        if part_of_speech.is_null() || category.is_null() || explanation.is_null() {
            println!(
                "Failed to display defination, here is the raw data:\n{}",
                raw.to_string()
            )
        }
        let temp_part_of_speech = part_of_speech.to_string();
        let temp_category = category.to_string();
        let temp_explanation = explanation.to_string();
        filtered.push(Defination {
            part_of_speech: (&temp_part_of_speech)[1..temp_part_of_speech.len() - 1].to_string(),
            category: (&temp_category)[1..temp_category.len() - 1].to_string(),
            explanation: (&temp_explanation)[1..temp_explanation.len() - 1].to_string(),
        })
    }
    filtered
}

fn get_data(collection: String) -> Value {
    let file_path = format!("C:\\Users\\shubh\\.dictionary\\{}.json", collection);
    match fs::read_to_string(file_path) {
        Ok(x) => return parse(&x, collection),
        Err(_) => {
            end(&format!(
                "data/{}.json not found, maybe the dictionary was deleted",
                collection
            ));
            return Value::Null;
        }
    }
}

fn parse(json_string: &str, collection: String) -> Value {
    match from_str(json_string) {
        Ok(y) => return y,
        Err(_) => {
            end(&format!(
                "{}.json failed to parse, maybe someone tampured the dictionary files",
                collection
            ));
            return Value::Null;
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
            end("Word must have atleast 1 character");
            return String::new();
        }
    }
}

fn max(x: usize, y: usize) -> usize {
    if x > y {
        return x;
    }
    y
}

fn pad(str: &String, new_len: &usize) -> String {
    str.to_owned() + &" ".repeat(new_len - str.len())
}

fn table_print(data: &Vec<Defination>) {
    let mut max_part_of_speech: usize = 0;
    let mut max_category: usize = 0;
    let mut max_explanation: usize = 0;
    for cell in data.iter() {
        max_part_of_speech = max(cell.part_of_speech.len(), max_part_of_speech);
        max_category = max(cell.category.len(), max_category);
        max_explanation = max(cell.explanation.len(), max_explanation);
    }
    max_part_of_speech += 2;
    max_category += 2;
    max_explanation += 2;
    if max_explanation > 95 {
        max_explanation = 95;
    }
    let barrier = format!(
        "+{}+{}+{}+",
        "-".repeat(max_part_of_speech),
        "-".repeat(max_category),
        "-".repeat(max_explanation)
    );
    println!("{}", barrier);
    for cell in data.iter() {
        if cell.explanation.len() > 93 {
            let mut segments = vec![];
            for (i, char) in cell.explanation.chars().enumerate() {
                if i / 93 >= segments.len() {
                    segments.push(String::new());
                }
                segments[i / 93] += &char.to_string();
            }
            print(
                &cell.part_of_speech,
                &cell.category,
                &segments[0],
                &max_part_of_speech,
                &max_category,
                &max_explanation,
            );
            for i in 1..segments.len() {
                let segment = &segments[i];
                print(
                    &String::new(),
                    &String::new(),
                    segment,
                    &max_part_of_speech,
                    &max_category,
                    &max_explanation,
                );
            }
            println!("{}", barrier);
            continue;
        }
        print(
            &cell.part_of_speech,
            &cell.category,
            &cell.explanation,
            &max_part_of_speech,
            &max_category,
            &max_explanation,
        );
        println!("{}", barrier);
    }
}

fn print(
    part_of_speech: &String,
    category: &String,
    explanation: &String,
    max_part_of_speech: &usize,
    max_category: &usize,
    max_explanation: &usize,
) {
    println!(
        "|{}|{}|{}|",
        pad(&part_of_speech, &max_part_of_speech),
        pad(&category, &max_category),
        pad(&explanation, &max_explanation),
    );
}
