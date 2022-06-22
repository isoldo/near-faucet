rm -r ./neardev
near dev-deploy ./target/wasm32-unknown-unknown/release/faucet.wasm
CONTRACT_ACCOUNT_ID=$(cat ./neardev/dev-account)
near call $CONTRACT_ACCOUNT_ID pay '{"amount": "180000000000000000000000000", "to": "'$1'"}' --accountId $CONTRACT_ACCOUNT_ID
