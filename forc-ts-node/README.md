# forc-tsgen

Example of a forc plugin written in Typescript with [Node.js runtime](https://nodejs.org/). This plugin requires npm to install.

## Installation

1. Clone this repository
2. Run `cd forc-tsgen && npm i && npm run build`
3. Run `npm install -g .`

## Usage

1. Check that it was installed. `forc-tsgen --version`
2. Check that forc recognizes the plugin. `forc plugins`
3. Use the plugin with forc. `forc tsgen --path sample-abi/sample-abi.json`