#!/usr/bin/env zsh

CWD=$(pwd)

WASM_PATH=$CWD/output/pacurarblog.wasm
WALLET_PEM=$CWD/dev-wallet.pem
WALLET_PEM_USER=$CWD/user-wallet.pem
CONTRACT_ADDRESS_DEVNET="erd1qqqqqqqqqqqqqpgqzq3sve9cavz8dq00smeul6ne8j0w6428eg8q9crjxs"
CONTRACT_ADDRESS_MAINNET="erd1qqqqqqqqqqqqqpgqq72p4p73sx6wkurl3epefzt4dkvwh7frvp3s73l9k6"

CONTRACT_ADDRESS=$CONTRACT_ADDRESS_DEVNET

deploy() {
    # --pem="${WALLET_PEM}" \
    mxpy contract deploy --recall-nonce \
        --bytecode="${WASM_PATH}" \
        --gas-limit=60000000 \
        "$@" || return
}

upgradeDev() {
    mxpy contract upgrade "$CONTRACT_ADDRESS" --recall-nonce \
        --bytecode="${WASM_PATH}" \
        --pem="${WALLET_PEM}" \
        --gas-limit=60000000 \
        --send || return
}

upgrade() {
    mxpy contract upgrade "$CONTRACT_ADDRESS" --recall-nonce \
        --bytecode="${WASM_PATH}" \
        --gas-limit=60000000 \
        --send || return
}

testCall() {
    mxpy contract call "$CONTRACT_ADDRESS" --recall-nonce \
        --function="buySpace" \
        --pem="${WALLET_PEM}" \
        --gas-limit=60000000 \
        --arguments str:top \
        --value 400000000000000000 \
        --send || return
}

testCallUser() {
    mxpy contract call "$CONTRACT_ADDRESS" --recall-nonce \
        --function="buySpace" \
        --pem="${WALLET_PEM_USER}" \
        --gas-limit=60000000 \
        --arguments str:top \
        --value 200000000000000000 \
        --send || return
}

withdraw() {
    mxpy contract call "$CONTRACT_ADDRESS" --recall-nonce \
        --function="withdraw" \
        --pem="${WALLET_PEM}" \
        --gas-limit=60000000 \
        --send || return
}

setAcceptedTokensDevnet() {
    mxpy contract call "$CONTRACT_ADDRESS" --pem dev-wallet.pem --recall-nonce \
    --gas-limit 5000000 --function addAcceptedTokens \
    --arguments str:USDT-188935 str:USDC-8d4068 \
    --send || return
}

setAcceptedTokensMainnet() {
    mxpy contract call "$CONTRACT_ADDRESS" --recall-nonce \
    --gas-limit 5000000 --function addAcceptedTokens \
    --arguments str:USDT-f8c08c str:USDC-c76f1f \
    "$@"|| return
}

# if there is no argument, list available commands
if [ $# -eq 0 ]; then
    echo "Usage: $0 <command> [args]"
    echo "Available commands:"
    echo "  deploy "
    exit 1
fi

# if the command exists, run it
if type "$1" 2>/dev/null | grep -q 'function'; then
    "$@"
else
    echo "Unknown command: $1"
    exit 1
fi
