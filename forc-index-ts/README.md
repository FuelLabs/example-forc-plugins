# forc-index-ts

Example of a forc plugin written in Typescript with [Deno runtime](https://deno.com/).

This plugin simply reads the `metadata.indexing.schema_path` from the `Forc.toml` file and displays the logged types.  
The code can potentially be extended to process the logged types to index the contract.

## Prerequisites

- [deno](https://docs.deno.com/runtime/getting_started/installation/)
- `forc` - Installed via
  [`fuelup`](https://docs.fuel.network/guides/contract-quickstart/)

## Installation

1. Clone this repository
2. Run the following - to globally install the plugin:

```sh
cd forc-index-ts/
deno install --global --force --allow-all forc-index.ts
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
