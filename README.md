 ```commandline             _      _                                 
███████╗██╗   ██╗██████╗ ████████╗███████╗███╗   ██╗███████╗ ██████╗ ██████╗ 
██╔════╝██║   ██║██╔══██╗╚══██╔══╝██╔════╝████╗  ██║██╔════╝██╔═══██╗██╔══██╗
███████╗██║   ██║██████╔╝   ██║   █████╗  ██╔██╗ ██║███████╗██║   ██║██████╔╝
╚════██║██║   ██║██╔══██╗   ██║   ██╔══╝  ██║╚██╗██║╚════██║██║   ██║██╔══██╗
███████║╚██████╔╝██████╔╝   ██║   ███████╗██║ ╚████║███████║╚██████╔╝██║  ██║
╚══════╝ ╚═════╝ ╚═════╝    ╚═╝   ╚══════╝╚═╝  ╚═══╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝
                                                                             
```

## System Requirements
* The binaries in ./bin/release are x86_64 binaries to be used with the Linux kernel.  
* Subtensor needs ~286 MiB to run.                      
* Architectures other than x86_64 are currently not supported.
* OSs other than Linux and MacOS are currently not supported.               

## Architectures
Subtensor support the following architectures:

## Linux x86_64
Requirements:
* Linux kernel 2.6.32+,
* glibc 2.11+

## MacOS x86_64
Requirements:
* MacOS 10.7+ (Lion+)

## Network requirements
* Subtensor needs access to the public internet
* Subtensor runs on ipv4
* Subtensor listens on the following ports:
1) 9944 - Websocket. This port is used by bittensor. It only accepts connections from localhost. Make sure this port is firewalled off from the public domain.
2) 9933 - RPC. This port is opened, but not used.
3) 30333 - p2p socket. This port accepts connections from other subtensor nodes. Make sure your firewall(s) allow incoming traffic to this port.

* It is assumed your default outgoing traffic policy is ACCEPT. If not, make sure outbound traffic to port 30333 is allowed.

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-subtensor -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-subtensor --dev
```

Purge the development chain's state:

```bash
./target/release/node-subtensor purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-subtensor -lruntime=debug --dev
```

### run debug with logs.

SKIP_WASM_BUILD=1 RUST_LOG=runtime=debug -- --nocapture

## Run with Docker :whale:
You can run an up to date Substrate blockchain using

```bash
docker-compose up
```
which will download the hourly blockchain snapshot and compile it into a docker container, then run it locally on your machine. 

You can use 
```bash
docker-compose up -d
```
to run the blockchain in the background. 


### Run with WSS

Use openssl to create cer and key files.

Be sure to replace YOUR_PASS_HERE to a secure password

```bash
sudo openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout subtensor.key -out subtensor.crt -config subtensor.conf -passin pass:YOUR_PASS_HERE
```

Export a pfx that you can import / trust

```bash
sudo openssl pkcs12 -export -out subtensor.pfx -inkey subtensor.key -in subtensor.crt 
```

then finally start the node

```bash
docker-compose up
```
