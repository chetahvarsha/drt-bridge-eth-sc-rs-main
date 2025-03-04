deployFaucet() {
    CHECK_VARIABLES FAUCET_WASM ALICE

    drtpy contract deploy --bytecode=${FAUCET_WASM} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=20000000 \
    --send --outfile=deploy-faucet-testnet.interaction.json --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(drtpy data parse --file="./deploy-faucet-testnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="./deploy-faucet-testnet.interaction.json" --expression="data['contractAddress']")

    drtpy data store --key=address-testnet-faucet --value=${ADDRESS}
    drtpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Faucet: ${ADDRESS}"
    update-config FAUCET ${ADDRESS}
}

setMintRoleForUniversalToken() {
  CHECK_VARIABLES ALICE ALICE_ADDRESS

  drtpy contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments str:${UNIVERSAL_TOKEN} ${ALICE_ADDRESS} str:DCDTRoleLocalMint \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

mintAndDeposit() {
  CHECK_VARIABLES ALICE ALICE_ADDRESS FAUCET

  read -p "Amount to mint (without decimals): " AMOUNT_TO_MINT
  VALUE_TO_MINT=$(echo "scale=0; $AMOUNT_TO_MINT*10^$NR_DECIMALS_UNIVERSAL/1" | bc)
  drtpy contract call ${ALICE_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=300000 --function="DCDTLocalMint" \
    --arguments str:${UNIVERSAL_TOKEN} ${VALUE_TO_MINT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}

  sleep 6

  drtpy contract call ${FAUCET} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=5000000 --function="DCDTTransfer" \
    --arguments str:${UNIVERSAL_TOKEN} ${VALUE_TO_MINT} str:deposit 100 \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unSetMintRoleForUniversalToken() {
    CHECK_VARIABLES ALICE ALICE_ADDRESS DCDT_SYSTEM_SC_ADDRESS

    drtpy contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="unSetSpecialRole" \
    --arguments str:${UNIVERSAL_TOKEN} ${ALICE_ADDRESS} str:DCDTRoleLocalMint \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

deployTestCaller() {
    CHECK_VARIABLES TEST_CALLER_WASM ALICE

    drtpy contract deploy --bytecode=${TEST_CALLER_WASM} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=20000000 \
    --send --outfile=deploy-test-caller.interaction.json --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(drtpy data parse --file="./deploy-test-caller.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(drtpy data parse --file="./deploy-test-caller.interaction.json" --expression="data['contractAddress']")

    drtpy data store --key=address-test-caller --value=${ADDRESS}
    drtpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Test caller: ${ADDRESS}"
    update-config TEST_CALLER ${ADDRESS}
}
