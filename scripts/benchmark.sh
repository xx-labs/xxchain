#!/bin/bash

args="--release --features=runtime-benchmarks"

mkdir -p weights/xxnetwork

echo "Running xx network Runtime benchmarks"

cargo run $args benchmark \
    --chain "xxnetwork-dev" \
    --list |\
  tail -n+2 |\
  cut -d',' -f1 |\
  uniq |\
  grep -v frame_benchmarking |\
  grep -v pallet_offences > "xxnetwork_pallets"

# For each pallet found in the previous command, run benches on each function
while read -r line; do
  pallet="$(echo "$line" | cut -d' ' -f1)";
  echo "Runtime: xxnetwork. Pallet: $pallet";
cargo run $args -- benchmark \
  --chain="xxnetwork-dev" \
  --steps=50 \
  --repeat=20 \
  --pallet="$pallet" \
  --extrinsic="*" \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output="./weights/xxnetwork/${pallet/::/_}.rs"
done < "xxnetwork_pallets"
rm "xxnetwork_pallets"
cp ./weights/xxnetwork/* ./runtime/xxnetwork/weights/*
rm -r weights/xxnetwork
