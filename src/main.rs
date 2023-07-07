use anyhow::Result;
use ethers::{
    contract::Contract,
    core::types::Address,
    prelude::{
        abigen,
        k256::sha2::digest::typenum::{uint, Min},
    },
    providers::{Provider, StreamExt, Ws},
};
use redis::Commands;
use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;
use std::fs::read_to_string;
use std::sync::Arc;
//abigen!(FakeNFT, "./abi.json"); // Symbolic to get any contract abi
// It's that possible? just getting the address | if verified of course

// Fake Nft Address
#[derive(Debug, Deserialize)]
struct ConfigFile {
    sc_name: String,
    abi_path: String,
    api_key: String,
    contract_address: String,
    redis_database: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config_variables = load_json("./config.json").await?;
    //Load the ABI
    let sc_id = &config_variables.sc_name;
    let path = &config_variables.abi_path.as_str();
    // Should this be solved with a try?
    abigen!(FakeNFT, "./abi.json");
    // Setup-provider
    let provider = Provider::<Ws>::connect(config_variables.api_key).await?;
    let client = Arc::new(provider);
    //Parse the contract address
    let address: Address = config_variables.contract_address.parse()?;
    let contract = sc_name::new(address, client);
    //listen_all_events(&contract).await?;
    Ok(())
}

async fn load_json(path_to_json: &str) -> Result<ConfigFile> {
    let json_str = read_to_string(path_to_json)?; // Read the file content
    let config_file: ConfigFile = serde_json::from_str(&json_str)?; // Parse the JSON content into ConfigFile
    println!("The config file is {:?}", config_file);
    Ok(config_file)
}

async fn listen_all_events(contract: &FakeNFT<Provider<Ws>>) -> Result<()> {
    // optionally sync from recent?
    let events = contract.events().from_block(3739350);
    let mut stream = events.stream().await?; // .take(1) only works for one

    while let Some(Ok(evt)) = stream.next().await {
        match evt {
            FakeNFTEvents::MintFilter(f) => process_mint_event(f).await?, // Let's see the structure tho , it can store everything to but i need my FK or PK
            FakeNFTEvents::TransferFilter(f) => println!("{f:?}"),
            FakeNFTEvents::OwnershipTransferredFilter(f) => println!("{f:?}"),
            FakeNFTEvents::ApprovalForAllFilter(f) => println!("{f:?}"),
            FakeNFTEvents::ApprovalFilter(f) => println!("{f:?}"),
            // This scopes only works once tho :(
        }
    }

    Ok(())
}

async fn process_mint_event(mint_filter: MintFilter) -> Result<()> {
    let addy = format!("{:?}", mint_filter.to);
    let tokenid = (mint_filter.token_id).to_string();

    store_in_redis(&addy, &tokenid).await?; // Wait cause of latency
    Ok(())
}

//Database connection
async fn store_in_redis(address: &str, value: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.1/")?; // <--- This will change if we dockerized it // redis crate
                                                           //client custom type , //open method associated with client
    let mut con = client.get_connection()?;

    //Since a address can hold multiple tokens ids we use LPUSH
    con.lpush(address, value)?;
    Ok(())
}

async fn get_block_sc(scblock: &u32) -> u32 {}
//There's any way to get getterts for all the Events?? what about auto writing it in to a file and then we can call it ?

//If it's a CLI we need to somehow store the PID Process number for killin it later

//Ideas
// What about get in what blokc does the smart contract gets deployed to stard monitorizing it
