deployBridgeProxy() {
    CHECK_VARIABLES PROXY_WASM MULTI_TRANSFER

    drtpy contract deploy --bytecode=${PROXY_WASM} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=200000000 \
    --arguments ${MULTI_TRANSFER} \
    --send --outfile="deploy-proxy-testnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(drtpy data parse --file="./deploy-proxy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(drtpy data parse --file="./deploy-proxy-testnet.interaction.json" --expression="data['contractAddress']")

    # drtpy data store --key=address-testnet-proxy --value=${ADDRESS}
    # drtpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Proxy contract address: ${ADDRESS}"
    update-config BRIDGE_PROXY ${ADDRESS}
}

setBridgedTokensWrapperOnSCProxy() {
    CHECK_VARIABLES BRIDGE_PROXY BRIDGED_TOKENS_WRAPPER

    drtpy contract call ${BRIDGE_PROXY} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="setBridgedTokensWrapperAddress" \
    --arguments ${BRIDGED_TOKENS_WRAPPER} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setMultiTransferOnSCProxy() {
    CHECK_VARIABLES BRIDGE_PROXY MULTI_TRANSFER

    drtpy contract call ${BRIDGE_PROXY} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="setMultiTransferAddress" \
    --arguments ${MULTI_TRANSFER} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setDcdtSafeOnSCProxy() {
    CHECK_VARIABLES BRIDGE_PROXY SAFE

    drtpy contract call ${BRIDGE_PROXY} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="setDcdtSafeAddress" \
    --arguments ${SAFE} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

deployBridgeProxyForUpgrade() {
    CHECK_VARIABLES PROXY_WASM MULTI_TRANSFER

    drtpy contract deploy --bytecode=${PROXY_WASM} --recall-nonce "${DRTPY_SIGN[@]}" \
        --gas-limit=200000000 \
        --arguments ${MULTI_TRANSFER} \
        --send --outfile="deploy-proxy-upgrade.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(drtpy data parse --file="./deploy-proxy-upgrade.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="./deploy-proxy-upgrade.interaction.json" --expression="data['contractAddress']")

    echo ""
    echo "New proxy contract address: ${ADDRESS}"
}

upgradeBridgeProxyContract() {
    local NEW_BRIDGE_PROXY_ADDR=$(drtpy data parse --file="./deploy-proxy-upgrade.interaction.json" --expression="data['contractAddress']")

    drtpy contract call ${MULTISIG} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=400000000 --function="upgradeChildContractFromSource" \
    --arguments ${BRIDGE_PROXY} ${NEW_BRIDGE_PROXY_ADDR} 0x00 \
    --send --outfile="upgrade-proxy-child-sc.json" --proxy=${PROXY} --chain=${CHAIN_ID}
}
