use anyhow::Result;
use dotenv::dotenv;
use ethers::{
    contract::Contract,
    core::types::Address,
    prelude::k256::sha2::digest::typenum::Min,
    providers::{Provider, StreamExt, Ws},
};
use redis::Commands;
use std::sync::Arc;
use std::{fs, path};


abigen!(FakeNFT, "./abi.json"); // Symbolic to get any contract abi
                                // It's that possible? just getting the address | if verified of course

// Fake Nft Address

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let mut SC_ADDRESS = std::env::var("CONTRACT_ADDRESS").expect("Failed to read Contract Address");
    let mut API_KEY = std::env::var("API_KEY")?;
    let mut SC_NAME = std::env::var("API_KEY")?;
    let mut PATH = std::env::var("PATH")?;
    let provider = Provider::<Ws>::connect(API_KEY).await?;
    let address : Address= SC_ADDRESS.parse()?;
    //Load SC
    let contract_abi = Contract::new(SC_ADDRESS,PATH,Arc::new(provider))

    abigen!(sc_name, contract_abi);

    let client = Arc::new(provider);
    //Parse the contract address
    let address: Address = SC_ADDRESS.parse()?;

    listen_all_events(&contract).await?;
    Ok(())
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
    let client = redis::Client::open("redis://127.0.1/")?; // <--- This will change if we dockerized it
    let mut con = client.get_connection()?;

    //Since a address can hold multiple tokens ids we use LPUSH
    con.lpush(address, value)?;
    Ok(())
}
//There's any way to get getterts for all the Events?? what about auto writing it in to a file and then we can call it ?

//If it's a CLI we need to somehow store the PID Process number for killin it later

