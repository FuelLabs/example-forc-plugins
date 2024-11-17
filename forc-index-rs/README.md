# forc-index-rs

Example of a forc plugin written in Rust.

This plugin simply reads the `metadata.indexing.schema_path` from the `Forc.toml` file and displays the logged types.  
The code can potentially be extended to process the logged types to index the contract.

## Prerequisites

- [rust](https://www.rust-lang.org/tools/install)
- `forc` - Installed via
  [`fuelup`](https://docs.fuel.network/guides/contract-quickstart/)

## Installation

1. Clone this repository
2. Run the following - to globally install the plugin:

```sh
cd forc-index-rs/
cargo install --path .
```

## Usage

1. Check that it was installed:

```sh
forc-index --version
```

2. Check that forc recognizes the plugin:

```sh
forc plugins
```

3. Use the plugin with forc:

```sh
forc index --path ./sample-project/Forc.toml
```
