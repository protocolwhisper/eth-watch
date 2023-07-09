use anyhow::Result;
use ethers::abi::Abi;
use ethers::prelude::k256::elliptic_curve::bigint::const_residue;
use ethers::prelude::{abigen, multicall_contract, Abigen, ContractInstance};
use ethers::{
    contract::Contract,
    core::types::Address,
    providers::{Provider, Ws},
};
use redis::Commands;
use serde::Deserialize;
use std::fmt::format;
use std::sync::Arc;

//abigen!(FakeNFT, "./abi.json"); // Symbolic to get any contract abi
// It's that possible? just getting the address | if verified of course
abigen!(MyContract, "./abi.json");

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
async fn main() -> Result<(), anyhow::Error> {
    let config_variables = load_config_file().await?;
    let contract_instance = create_contract_instance(
        &config_variables.api_key,
        &config_variables.contract_address,
        &config_variables.abi_path,
    )
    .await?;
    listen_all_events(contract_instance).await?;
    Ok(())
}

// Load ConfigFile
async fn load_config_file() -> Result<ConfigFile> {
    let path_to_json = "./config.json";
    let json_str = std::fs::read_to_string(path_to_json)?; // Read the file content
    let config_file: ConfigFile = serde_json::from_str(&json_str)?; // Parse the JSON content into the Struct
    println!("The config file is {:?}", config_file);
    Ok(config_file)
}

// Instanciate Contract
async fn create_contract_instance(
    provider_url: &str,
    contract_address: &str,
    abi_path: &str,
) -> Result<Contract<Provider<Ws>>> {
    // Parse the contract address
    let address: Address = contract_address.parse()?;

    // Load the ABI
    let abi: Abi = serde_json::from_reader(std::fs::File::open(abi_path)?)?;

    // Create an instance of the provider
    let provider = Provider::<Ws>::connect(provider_url).await?;
    let client = Arc::new(provider); // Express a piece of data has multiple owners
                                     //Arc::clone(&client);

    // Create the contract instance
    let contract = Contract::new(address, abi, client);

    Ok(contract)
}

async fn listen_all_events(
    contract: ContractInstance<Arc<Provider<Ws>>, Provider<Ws>>,
) -> Result<()> {
    let events = contract.event()
    // optionally sync froeventnt?
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

/* async fn process_mint_event(mint_filter: MintFilter) -> Result<()> {
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

async fn get_block_sc(scblock: &u32) -> u32 {} */
//There's any way to get getterts for all the Events?? what about auto writing it in to a file and then we can call it ?

//If it's a CLI we need to somehow store the PID Process number for killin it later

//Ideas
// What about get in what blokc does the smart contract gets deployed to stard monitorizing it
