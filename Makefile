#######################
### xx network only ###
#######################

# Builds xxnetwork-chain binary containing only xxnetwork runtime
build-release:
	@cargo build -p xxnetwork-cli --release

# TODO: add build optimizations for production binary

# Builds xxnetwork-chain binary containing only xxnetwork runtime with the try-runtime feature
build-release-try-runtime:
	@cargo build -p xxnetwork-cli --release --features try-runtime

#######################
###  xx canary only ###
#######################

# Builds xxnetwork-chain binary containing only canary runtime
build-canary-release:
	@cargo build -p xxnetwork-cli --release --no-default-features --features cli,canary

# Builds xxnetwork-chain binary containing only canary runtime with the try-runtime feature
build-canary-release-try-runtime:
	@cargo build -p xxnetwork-cli --release --no-default-features --features cli,canary,try-runtime

# TODO: add build optimizations for production binary

#######################
###  both runtimes  ###
#######################

# Build all packages
build:
	@cargo build --release --features canary

# Builds xxnetwork-chain binary containing accelerated xxnetwork and canary runtime
build-dev:
	@cargo build -p xxnetwork-cli --release --features canary,fast-runtime

# Builds xxnetwork-chain binary containing accelerated xxnetwork and canary runtime with the try-runtime feature
build-dev-try-runtime:
	@cargo build -p xxnetwork-cli --release --features canary,fast-runtime,try-runtime

#######################
###  build runtimes ###
#######################

build-canary-runtime:
	@srtool build --package canary-runtime

build-xxnetwork-runtime:
	@srtool build --package xxnetwork-runtime

build-runtimes: build-canary-runtime build-xxnetwork-runtime


build-linux-from-mac:
	@echo -e "Before proceeding make sure you check README\n"
	@CC_x86_64_unknown_linux_gnu="x86_64-unknown-linux-gnu-gcc" CXX_x86_64_unknown_linux_gnu="x86_64-unknown-linux-gnu-g++" CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="x86_64-unknown-linux-gnu-gcc" cargo build --release --target=x86_64-unknown-linux-gnu

#######################
###      tests      ###
#######################

all-tests:
	@echo "Running all unit tests\n"
	@cargo test

test-pallets:
	@echo "Running unit tests for all pallets\n"
	@cd chainbridge; cargo test; cd ../
	@cd claims; cargo test; cd ../
	@cd swap; cargo test; cd ../
	@cd xx-betanet-rewards; cargo test; cd ../
	@cd xx-cmix; cargo test; cd ../
	@cd xx-economics; cargo test; cd ../
	@cd xx-public; cargo test; cd ../
	@cd xx-team-custody; cargo test; cd ../
