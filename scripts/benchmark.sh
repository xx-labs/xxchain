
for pallet in xx-cmix xx-team-custody xx-economics swap
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
    --output=./$pallet/src/weights.rs \
    --template=./scripts/frame-weight-template.hbs
done