# rrpc

Rust command-line application designed to simplify the process of retrieving RPC configurations .

# RRPC Tool

## Description

RRPC Tool is a simple Rust command-line application designed to simplify the process of retrieving RPC configurations for various blockchain networks. It allows users to easily access RPC URLs stored in a configuration file within their home directory, making it an essential utility for blockchain developers and enthusiasts who frequently interact with different networks.

## Features

- Easy retrieval of RPC URLs for various blockchain networks.
- Simple configuration via a single `.rrpc.config` file.
- Cross-platform support with straightforward setup.

## Installation

### Prerequisites

Ensure you have Rust installed on your system. If Rust is not installed, you can follow the installation instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Installing RRPC Tool

1. Clone the repository:

   ```bash
   git clone https://github.com/RosalinaLuma620/rrpc.git
   cd rrpc-tool
   ```

2. Build and install the tool using Cargo:

   ```bash
   cargo install --path .
   ```

   This command compiles the project and installs the `rrpc` binary to `~/.cargo/bin`, which should be in your `PATH` if you have Rust installed correctly.

## Configuration

Before using the RRPC Tool, you must create a `.rrpc.config` file in your home directory. This file will contain the RPC URLs you wish to access. Here's how to set it up:

1. Open your terminal or command prompt.
2. Use a text editor to create the `.rrpc.config` file in your home directory:

   ```bash
   touch ~/.rrpc.config
   ```

3. Edit the `.rrpc.config` file to include your RPC configurations. Use the format `network_name=rpc_url` for each entry. For example:

   ```
   arbitrum-sepolia1=https://arb-sepolia.g.alchemy.com/v2/yourapikey
   arbitrum-one=https://arb1.arbitrum.io/rpc
   ethereum-mainnet=https://mainnet.infura.io/v3/yourapikey
   ```

## Usage

After installing the RRPC Tool and setting up the `.rrpc.config` file, you can easily retrieve RPC URLs by running the tool from the command line:

```bash
rrpc

```
