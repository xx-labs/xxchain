#!/bin/bash

make build-bench

mkdir -p weights

echo "Running xx network Runtime benchmarks"

./target/production/xxnetwork-chain benchmark pallet \
    --chain "dev" \
    --list |\
  tail -n+2 |\
  cut -d',' -f1 |\
  uniq > "xxnetwork_pallets"

# For each pallet found in the previous command, run benches on each function
while read -r line; do
  pallet="$(echo "$line" | cut -d' ' -f1)";
  echo "Runtime: xxnetwork. Pallet: $pallet";
./target/production/xxnetwork-chain benchmark pallet \
  --chain="dev" \
  --steps=50 \
  --repeat=20 \
  --pallet="$pallet" \
  --extrinsic="*" \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output="./weights/${pallet/::/_}.rs"
done < "xxnetwork_pallets"
rm "xxnetwork_pallets"
