use ethers::abi::{Abi, RawLog};
use ethers::addressbook::Contract;
use ethers::contract::decode_logs;

use ethers::types::{Bytes, Log, H256};
use ethers::{
    providers::{Http, Middleware, Provider, StreamExt, Ws},
    types::{Address, Filter},
};
use eyre::Result;
use std::fs::{self, File};
use std::io::Read;
use std::sync::Arc;

struct decode_log {
    topics: Vec<H256>,
    data: Bytes,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Ws>::connect("wss://warmhearted-patient-market.ethereum-sepolia.discover.quiknode.pro/aabb295fef34112a987fa071899d1479e592bce5/").await?;
    let filter = Filter::new().address(
        "0x06211D152669996d6756D09E6257847Fb37B1Df5"
            .parse::<Address>()
            .unwrap(),
    );

    let mut stream = provider.subscribe_logs(&filter).await?; //  "?" IF it exist? , using for results (.unwrap()) propagate errors from the stack to your result enum Result<T,E>
                                                              //Closure? Like an arrow function :)
                                                              //8*2

    // Your code to handle logs goes here.

    while let Some(log) = stream.next().await {
        let topics = log.topics;
        let data = log.data;
        let raw: RawLog = log.into();

        // match log{} //  Enum involve , to handle each of the cases that the enum can have :) patter matching
        println!("New log: {:?}", log);
    }

    //ethers::types::Log
    fn get_events(contract: String) {
        //Call the contract and then extract the hash of them by getting only their type
    }

    //TO-DO
    fn decode_events(log: Log) {
        //From abi take the events and hast th
    }
    // Extract events from presumly topi[0]
    Ok(())
}
