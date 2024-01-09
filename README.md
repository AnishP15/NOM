# Simple Option

Tutorial: https://docs.cosmwasm.com/tutorials/simple-option/intro


# Compiling contract

cargo build --release

# Example Upload

nibid tx wasm store simple_option.wasm --from validator --chain-id nibiru-testnet-1

# Example Instantiate 

nibid tx wasm instantiate 1 '{"counter_offer": [{"denom": "uscrt", "amount": "1000"}], "expires": 1000}' --label "simple option contract" --no-admin --from validator
