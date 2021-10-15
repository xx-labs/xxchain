pallets=("swap" "xx-cmix" "xx-team-custody" "xx-economics")
mkdir weights
for pallet in ${pallets[@]}
do
  cargo run --release \
    --features=runtime-benchmarks \
    --manifest-path=cli/Cargo.toml \
    -- \
    benchmark \
    --chain=xxnetwork-dev \
    --steps=50 \
    --repeat=20 \
    --pallet=$pallet \
    --extrinsic="*" \
    --execution=wasm \
    --wasm-execution=compiled \
    --heap-pages=4096 \
    --output=./weights/$pallet-weights.rs \
    --template=./scripts/frame-weight-template.hbs
done
for pallet in ${pallets[@]}
do
    cp -f ./weights/$pallet-weights.rs ./$pallet/src/weights.rs
done
rm -r weights
