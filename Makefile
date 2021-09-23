build:
	@cargo build --release

build-phoenixx-runtime:
	@srtool build --package phoenixx-runtime

build-protonet-runtime:
	@srtool build --package protonet-runtime

build-xxnetwork-runtime:
	@srtool build --package xxnetwork-runtime

build-runtimes: build-phoenixx-runtime build-protonet-runtime build-xxnetwork-runtime

build-linux-from-mac:
	@echo -e "Before proceeding make sure you check README\n"
	@CC_x86_64_unknown_linux_gnu="x86_64-unknown-linux-gnu-gcc" CXX_x86_64_unknown_linux_gnu="x86_64-unknown-linux-gnu-g++" CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="x86_64-unknown-linux-gnu-gcc" cargo build --release --target=x86_64-unknown-linux-gnu

test-pallets:
	@echo -e "Running unit tests for all pallets\n"
	@cd chainbridge; cargo test; cd ../
	@cd claims; cargo test; cd ../
	@cd staking; cargo test; cd ../
	@cd swap; cargo test; cd ../
	@cd xx-cmix; cargo test; cd ../
	@cd xx-economics; cargo test; cd ../
	@cd xx-team-custody; cargo test; cd ../
