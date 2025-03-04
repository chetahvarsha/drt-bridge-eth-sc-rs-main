deployMultiTransfer() {
    CHECK_VARIABLES MULTI_TRANSFER_WASM

    drtpy contract deploy --bytecode=${MULTI_TRANSFER_WASM} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=100000000 --metadata-payable \
    --send --outfile="deploy-multitransfer-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    ADDRESS=$(drtpy data parse --file="./deploy-multitransfer-testnet.interaction.json" --expression="data['contractAddress']")
    drtpy data store --key=address-testnet-multitransfer --value=${ADDRESS}

    echo ""
    echo "Multi transfer contract address: ${ADDRESS}"
    update-config MULTI_TRANSFER ${ADDRESS}
}

setBridgeProxyContractAddressOnMultiTransfer() {
    CHECK_VARIABLES MULTI_TRANSFER BRIDGE_PROXY

    drtpy contract call ${MULTI_TRANSFER} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="setBridgeProxyContractAddress" \
    --arguments ${BRIDGE_PROXY} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setBridgedTokensWrapperOnMultiTransfer() {
    CHECK_VARIABLES MULTI_TRANSFER BRIDGED_TOKENS_WRAPPER

    drtpy contract call ${MULTI_TRANSFER} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="setWrappingContractAddress" \
    --arguments ${BRIDGED_TOKENS_WRAPPER} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

deployMultiTransferForUpgrade() {
    CHECK_VARIABLES MULTI_TRANSFER_WASM

    drtpy contract deploy --bytecode=${MULTI_TRANSFER_WASM} --recall-nonce "${DRTPY_SIGN[@]}" \
        --gas-limit=100000000 --metadata-payable \
        --send --outfile="deploy-multitransfer-upgrade.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(drtpy data parse --file="./deploy-multitransfer-upgrade.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="./deploy-multitransfer-upgrade.interaction.json" --expression="data['contractAddress']")

    echo ""
    echo "New multi transfer contract address: ${ADDRESS}"
}

upgradeMultiTransferContract() {
    local NEW_MULTI_TRANSFER_ADDR=$(drtpy data parse --file="./deploy-multitransfer-upgrade.interaction.json" --expression="data['contractAddress']")

    drtpy contract call ${MULTISIG} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=400000000 --function="upgradeChildContractFromSource" \
    --arguments ${MULTI_TRANSFER} ${NEW_MULTI_TRANSFER_ADDR} 0x00 \
    --send --outfile="upgrade-multitransfer-child-sc.json" --proxy=${PROXY} --chain=${CHAIN_ID}
}
