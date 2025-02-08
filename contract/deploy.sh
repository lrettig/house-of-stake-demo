#!/bin/bash
set -e

NETWORK=${1:-testnet}
ACCOUNT_ID=${2:-$NEAR_ACCOUNT_ID}

if [ -z "$ACCOUNT_ID" ]; then
    echo "ERROR: NEAR_ACCOUNT_ID environment variable is required"
    exit 1
fi

cd "$(dirname $0)"

# Deploy veNEAR contract
VENEAR_ID="venear.$ACCOUNT_ID"
near deploy --accountId $ACCOUNT_ID --wasmFile res/venear_contract.wasm
near call $VENEAR_ID new "{\"owner_id\": \"$ACCOUNT_ID\"}" --accountId $ACCOUNT_ID

# Deploy voting contract
VOTING_ID="voting.$ACCOUNT_ID"
near deploy --accountId $ACCOUNT_ID --wasmFile res/voting_contract.wasm
near call $VOTING_ID new "{\"owner_id\": \"$ACCOUNT_ID\", \"ve_token\": \"$VENEAR_ID\"}" --accountId $ACCOUNT_ID 