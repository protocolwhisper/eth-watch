use ethers::contract::Abigen;
use ethers::utils::{hex, keccak256};
use hex::encode;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub struct Event {
    name: String,
    parameters: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let contract_name: &str = "FakeNFT";
    let output_dir: &str = "./src";

    let output_path = Path::new(output_dir).join("token.rs");

    let abi_path = "./abi.json";
    let _events = get_events_from_abi(abi_path)?;

    match Abigen::new(contract_name, abi_path)?
        .generate()?
        .write_to_file(output_path)
    {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    }

    let events = match get_events_from_abi(abi_path) {
        Ok(events) => events,
        Err(e) => return Err(e.into()),
    };

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("src/events.rs")?;

    write!(file, "pub const MY_EVENTS: &[&str] = & [\n")?;
    for event in &events {
        let params = event.parameters.join(",");
        write!(file, "\"{}({})\",\n", event.name, params)?;
    }
    write!(file, "];\n\n")?;

    write!(file, "pub const MY_EVENTS_HASHED: &[&str] = & [\n")?;
    for event in &events {
        let params = event.parameters.join(",");
        let event_string = format!("{}({})", event.name, params);
        let hashed = keccak256(event_string);
        let hashed_str = format!("0x{}", encode(hashed)); // Convert hashed value to hex string with "0x" prefix
        write!(file, "\"{}\",\n", hashed_str)?;
    }
    write!(file, "];\n")?;

    Ok(())
}

fn get_events_from_abi(abi_path: &str) -> Result<Vec<Event>, Box<dyn Error>> {
    let abi = fs::read_to_string(abi_path)?;

    let v: Value = serde_json::from_str(&abi)?;

    let mut events = Vec::new();
    for item in v.as_array().unwrap() {
        if item["type"] == "event" {
            let event_name = item["name"].as_str().unwrap().to_string();
            let mut event_params = Vec::new();
            for input in item["inputs"].as_array().unwrap() {
                let param_type = input["type"].as_str().unwrap().to_string();
                event_params.push(param_type);
            }
            events.push(Event {
                name: event_name,
                parameters: event_params,
            });
        }
    }

    Ok(events)
}
