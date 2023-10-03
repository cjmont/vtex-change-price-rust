# Vtex Rust Hourly Task

This project contains a scheduled task in Rust that runs every hour to update prices on an external API.

## Features

- Filters and processes price data.
- Makes HTTP requests to the VTEX API.
- Logs API responses for auditing purposes.

## Requirements

- Rust 1.50 or higher.
- Cargo (comes with Rust).

## Installation

1. Clone this repository:

```bash
git clone https://github.com/cjmont/vtex-change-price-rust.git
cd vtex-change-price-rust
```

2. Build the project:

```bash
cargo build --release
```

## Usage

To run the scheduled task:

```bash
cargo run --release
```

This will start the task, which will run every hour.

## Core Structures

- `Price`: Represents pricing information for a product.
- `Sku`: Contains identity and associated product.
- `Product`: Represents a product and contains merchant information.
- `Merchant`: Represents merchant information and contains sets of accounts and identities.
- `Account`: Account information for the VTEX API.
- `Store`: Represents a store and contains identities.
- `Identity`: Identity information used across various structures.
- `Log`: Used to log API responses.

## Contributing

If you wish to contribute to the project, please fork the repository and submit your changes via a pull request.

## License

This project is under the MIT license. See the `LICENSE` file for more details.
