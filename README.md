# xx network Substrate based blockchain node

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### MacOS users: setup to compile for Linux

Before being able to build for linux on macOS, the following extra steps are needed:

```sh
rustup target add x86_64-unknown-linux-gnu # Install linux GNU rust target
brew tap SergioBenitez/osxct               # Tap this project with brew
brew install x86_64-unknown-linux-gnu      # Install cross-compile tools for GNU
```

### Build

The makefile provides build commands, of which the most important are:

```sh
make build                # Build node binary for current OS
make build-linux-from-mac # Build node binary for Linux from macOS
```

### Test

The makefile provides the `test-pallets` command which runs unit tests for all custom pallets, as follows:

```sh
chainbridge
claims
staking
swap
xx-cmix
xx-economics
xx-team-custody
```

### Benchmarking

Included is a script that automatically runs the benchmarking code and calculates extrinsic weights for relevant pallets.

This can be executed with:

```sh
sh ./scripts/benchmark.sh
```

### Code Review

A code review was performed by ChainSafe. It focused mostly on the modification to the Staking pallet
and the custom made xx network pallets for cmix, economics and team custody.
The report can be found [here](./doc/ChainSafe%20xxchain%20Code%20Review.pdf).
