use anyhow::Result;
use ethers::abi::Abi;
use ethers::contract::{Contract, ContractError};
use ethers::prelude::ContractInstance;
use ethers::prelude::LogMeta;
use ethers::prelude::*;
use ethers::providers::{Middleware, Provider};
use ethers::types::{Filter, Log};
use ethers::{core::types::Address, providers::Ws};
use futures::future::ok;
use futures::stream::StreamExt;
use redis::Commands;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::fmt::format;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
mod extract_Events; // Import our module (Name of the file)

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
    let events = extract_Events::extract_event_names(&config_variables.abi_path).unwrap(); // Get events from ABI

    listen_to_raw_logs(contract_instance, &config_variables.api_key).await?;
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
async fn handle_log(log: Log) -> Result<(), Box<dyn Error>> {
    println!("New log: {:?}", log);

    // Access the log fields
    let address = log.address;
    let topics = log.topics;
    let data = log.data;
    let block_hash = log.block_hash;
    let block_number = log.block_number;
    let transaction_hash = log.transaction_hash;
    let transaction_index = log.transaction_index;
    let log_index = log.log_index;
    let transaction_log_index = log.transaction_log_index;
    let log_type = log.log_type;
    let removed = log.removed;

    // Process the log data as needed
    // ...

    Ok(())
}

async fn listen_to_raw_logs(
    contract: ContractInstance<Arc<Provider<Client>>, Provider<Client>>,
    provider_url: &str,
) -> Result<(), Box<dyn Error>> {
    // create a filter for the contract
    let filter = Filter::new().address(vec![contract.address()]);

    // get the underlying provider
    let provider = Provider::<Ws>::connect(provider_url).await?;
    let client = Arc::new(provider);

    // get the logs
    let logs = match client.get_logs(&filter).await {
        Ok(logs) => logs,
        Err(err) => return Err(err.into()),
    };

    // process each log
    for log in logs {
        handle_log(log).await?;
    }

    Ok(())
}
