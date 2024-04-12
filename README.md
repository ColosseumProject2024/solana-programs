# DeVolt Solana Programs

![DeVolt](assets/logo.jpg)

Welcome to the DeVolt Solana Programs repository! This collection of Solana program powers the DeVolt platform, enabling decentralized energy management and auctioning functionalities.

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Programs](#programs)
  - [Battery Report](#battery-report)
  - [Place Bid](#place-bid)
- [Development](#development)
  - [Building](#building)
  - [Testing](#testing)
- [Scripts](#scripts)
- [SDK](#sdk)
- [Contributing](#contributing)
- [License](#license)
- [Documentation](#documentation)

## Overview

DeVolt's Solana programs offer a robust infrastructure for managing energy-related transactions on the blockchain. These contracts are essential for supporting the platform's dynamic and user-focused features.

## Getting Started

To set up your local development environment, follow these steps.

### Prerequisites

- Install Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- Install Solana CLI: [https://docs.solana.com/cli/install-solana-cli-tools](https://docs.solana.com/cli/install-solana-cli-tools)
- Install Anchor: [https://www.anchor-lang.com/](https://www.anchor-lang.com/)

### Installation

Clone the repository and build the program:

```bash
git clone https://github.com/devolthq/solana-programs.git
cd solana-programs
anchor build
```

## Programs

Find the program within the `programs/devolt/src` directory, with detailed functionalities outlined as follows.

### Battery Report

The `battery_report` program allows stations to submit updates on the status of their batteries. This information is critical for the efficient management of energy within the DeVolt platform.

Located at: `programs/devolt/src/instructions/battery_report.rs`.

Functionalities:
- Authenticates the station with the signer's key.
- Updates the station's data, such as identification and battery status.
- Concludes auctions based on the current time and selects winning bids.
- Automatically creates new auctions when the station reaches a certain energy deficit.

This program is a vital part of DeVolt's business logic, ensuring that the supply and demand for energy are met efficiently and transparently.

### Place Bid

The `place_bid` program is designed to allow bidders to make offers in energy auctions on the DeVolt platform.

Located at: `programs/devolt/src/instructions/place_bid.rs`.

Functionalities:
- Allows bidders to place bids in energy auctions.
- Registers the bid with bidder information, the desired amount of energy, and the offered price.
- Stores the bids in the station's account for evaluation upon auction conclusion.

This program ensures that the auction process is conducted fairly and orderly, allowing the station to find the best available energy offers.

## Development

### Building

Build the program and update the TypeScript SDK types with:

```bash
yarn build
```

Followed by:

```bash
yarn update-types
```

### Testing

Run tests with the following command:

```bash
cd sdk
```

Then:
```bash
yarn
```

And finally:
```bash
yarn start
```

## Scripts

Useful scripts for development are located in the `package.json` files:

- `lint:fix` - Auto-fix and format code with Prettier.
- `lint` - Check the code formatting with Prettier.
- `update-types` - Update the TypeScript SDK types after building.
- `build` - Build contracts and update SDK types.
- `anchor-tests` - Run tests for the program.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for the contribution process.

## License

This project is under the ISC license for root directory code and MIT license for the SDK. See the [LICENSE.md](LICENSE) file in the respective directories for details.

## Documentation

For more information on Anchor, the framework used for DeVolt program, visit the official [Anchor documentation](https://www.anchor-lang.com/).