## Run

## Build

`cargo build --target wasm32v1-none --release`
`cargo build --target wasm32-unknown-unknown --release`

`cargo build --manifest-path contracts/base/Cargo.toml --target wasm32-unknown-unknown --release --target-dir ./build/base`
`cargo build --manifest-path contracts/base/Cargo.toml --target wasm32v1-none --release --target-dir ./build/base`

## Run

`yarn run test:integration test/integration/contracts/toolbox/base.test.ts`

## Debug

`tail -f smartnet/config/debug.log 2>&1 | grep -E --color=always 'WAMR|ContractError|Publishing ledger [0-9]+'`