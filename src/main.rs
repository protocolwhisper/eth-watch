use ethers::abi::{Abi, RawLog};
use ethers::addressbook::Contract;
use ethers::contract::decode_logs;
use ethers::types::{Bytes, Log, H256};
use ethers::utils::hex::encode;
use ethers::{
    providers::{Http, Middleware, Provider, StreamExt, Ws},
    types::{Address, Filter},
};
use eyre::Result;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

mod events;

struct decode_log {
    topics: Vec<H256>,
    data: Bytes,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect("wss://distinguished-wiser-dawn.ethereum-sepolia.discover.quiknode.pro/657a528a840ac3fce543d6b7b1ef59b4e9a2006b/").await?;
    let filter = Filter::new().address(
        "0x06211D152669996d6756D09E6257847Fb37B1Df5"
            .parse::<Address>()
            .unwrap(),
    );

    let mut stream = provider.subscribe_logs(&filter).await?; //  "?" IF it exist? , using for results (.unwrap()) propagate errors from the stack to your result enum Result<T,E>
                                                              //Closure? Like an arrow function :)
                                                              //8*2

    let my_events = events::MY_EVENTS;
    let my_events_hashed = events::MY_EVENTS_HASHED;
    // Your code to handle logs goes here.
    let mut counter: i32 = 0;
    while let Some(log) = stream.next().await {
        let events_number = my_events.len();
        let raw: RawLog = log.into();
        counter += 1;

        // match log{} //  Enum involve , to handle each of the cases that the enum can have :) patter matching
        println!("New log({:?}): {:?}", &counter, &raw);
        if let Some(first_topic) = raw.topics.get(0) {
            let topic_str = format!("0x{}", encode(first_topic.as_bytes()));
            if let Some(pos) = my_events_hashed.iter().position(|&x| x == topic_str) {
                println!("The position of the topic in MY_EVENTS_HASHED: {}", pos);
                println!("Corresponding event in MY_EVENTS: {}", my_events[pos]);
            } else {
                println!("The topic does not exist in MY_EVENTS_HASHED");
            }
        } else {
            println!("Topics vector is empty");
        }
    }

    //ethers::types::Log
    // fn get_event_data() -> Option<(Vec<&'static str>, Vec<&'static str>)> {}

    //TO-DO
    fn decode_events(log: Log) {
        //From abi take the events and hast th
        let my_events = events::MY_EVENTS;
        let my_events_hashed = events::MY_EVENTS_HASHED;
    }
    // Extract events from presumly topi[0]
    Ok(())
}
