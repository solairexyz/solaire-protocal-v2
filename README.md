# solaire-protocal-v2

This project is a solaire protocol version 2. It is a completely decentralized ecommerce protocol run on top of ethereum layer 2s. The original version 1 can be found on https://api.doc.mysolaire.xyz 

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Installation

1. `cd solaire-protocol-v2`
2. `cargo build`

### Run the project

1. `cargo run`

## Features

* Buy: Allows users to purchase products.
* Sell: Enables users to list products for sale, including NFT-associated products.
* Delivery Service: Manages delivery status and information.
* Product Detail: Provides detailed information about products, including NFT token and contract addresses.

## Technology Stack

- Rust: Programming language used for building the backend.
- Actix-Web: Web framework for building the HTTP API.
- SQLx: Async SQL toolkit for Rust, used for database interactions.
- MySQL: Database for storing application data.