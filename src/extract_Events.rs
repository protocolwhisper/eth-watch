use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn extract_event_names(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // Open the file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Parse the JSON
    let abi: Value = serde_json::from_reader(reader)?;

    // Regular expression to match event declarations
    let re = Regex::new(r#""type"\s*:\s*"event""#)?;

    // Filter out the events and get their names
    let events: Vec<String> = abi
        .as_array()
        .unwrap()
        .iter()
        .filter(|item| re.is_match(&item.to_string()))
        .map(|item| item["name"].as_str().unwrap().to_string())
        .collect();

    Ok(events)
}

fn main() {
    match extract_event_names("../abi.json") {
        Ok(events) => {
            for event in events {
                println!("{}", event);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
