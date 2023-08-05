# eth-watch :mag: :globe_with_meridians:

## Introduction :book:

eth-watch is a robust tool built with Rust :crab: that provides the ability to listen and index Ethereum smart contract events. It leverages the potent decoding capabilities of the eth-rs library, offering a seamless way to query smart contract events.

## Key Features :key:

- **Event Listening** :ear: : eth-watch starts to listen for all events from a specified smart contract.
- **Indexing** :file_folder: : All smart contract events are indexed in a Postgres database, simplifying data retrieval and manipulation.
- **Querying** :mag_right: : With the data indexed in Postgres, querying for specific events or conditions becomes a straightforward task.
- **Rust Power** :muscle: : By leveraging Rust's performance and memory safety features, eth-watch provides a reliable and efficient solution for smart contract event handling.

## How it works :wrench:

The project utilizes the eth-rs library for interacting with the Ethereum blockchain and for decoding smart contract events. These events are then stored in a Postgres database for easier querying.

## To-Do :pencil:

- [x] Get events from ABI
- [x] Listen to smart contract logs
- [x] Decode logs for event signatures

## Stay Tuned :satellite:

This is a learning project and currently under active development :construction_worker: . More features will be added as the project progresses. Contributions and suggestions are welcome! :heart:
