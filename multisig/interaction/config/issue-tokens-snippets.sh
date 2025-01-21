DCDT_ISSUE_COST=50000000000000000

issueUniversalToken() {
    CHECK_VARIABLES DCDT_SYSTEM_SC_ADDRESS DCDT_ISSUE_COST UNIVERSAL_TOKEN_DISPLAY_NAME \
    UNIVERSAL_TOKEN_TICKER NR_DECIMALS_UNIVERSAL

    drtpy contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --value=${DCDT_ISSUE_COST} --function="issue" \
    --arguments str:${UNIVERSAL_TOKEN_DISPLAY_NAME} str:${UNIVERSAL_TOKEN_TICKER} \
    0 ${NR_DECIMALS_UNIVERSAL} str:canAddSpecialRoles str:true \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

issueChainSpecificToken() {
    CHECK_VARIABLES DCDT_SYSTEM_SC_ADDRESS DCDT_ISSUE_COST CHAIN_SPECIFIC_TOKEN_DISPLAY_NAME \
    CHAIN_SPECIFIC_TOKEN_TICKER NR_DECIMALS_CHAIN_SPECIFIC UNIVERSAL_TOKENS_ALREADY_MINTED
    
    VALUE_TO_MINT=$(echo "scale=0; $UNIVERSAL_TOKENS_ALREADY_MINTED*10^$NR_DECIMALS_CHAIN_SPECIFIC/1" | bc)

    drtpy contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --value=${DCDT_ISSUE_COST} --function="issue" \
    --arguments str:${CHAIN_SPECIFIC_TOKEN_DISPLAY_NAME} str:${CHAIN_SPECIFIC_TOKEN_TICKER} \
    ${VALUE_TO_MINT} ${NR_DECIMALS_CHAIN_SPECIFIC} str:canAddSpecialRoles str:true \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

transferToSC() {
    CHECK_VARIABLES BRIDGED_TOKENS_WRAPPER CHAIN_SPECIFIC_TOKEN

    VALUE_TO_MINT=$(echo "scale=0; $UNIVERSAL_TOKENS_ALREADY_MINTED*10^$NR_DECIMALS_CHAIN_SPECIFIC/1" | bc)

    drtpy contract call ${BRIDGED_TOKENS_WRAPPER} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=5000000 --function="DCDTTransfer" \
    --arguments str:${CHAIN_SPECIFIC_TOKEN} ${VALUE_TO_MINT} str:depositLiquidity \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setMintRole() {
    drtpy contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="setSpecialRole" \
    --arguments str:${CHAIN_SPECIFIC_TOKEN} ${ALICE_ADDRESS} str:DCDTRoleLocalMint \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

unSetMintRole() {
    drtpy contract call ${DCDT_SYSTEM_SC_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=60000000 --function="unSetSpecialRole" \
    --arguments str:${CHAIN_SPECIFIC_TOKEN} ${ALICE_ADDRESS} str:DCDTRoleLocalMint \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

mint() {
    CHECK_VARIABLES NR_DECIMALS_CHAIN_SPECIFIC ALICE_ADDRESS CHAIN_SPECIFIC_TOKEN
    read -p "Amount to mint(without decimals): " AMOUNT_TO_MINT
    VALUE_TO_MINT=$(echo "scale=0; $AMOUNT_TO_MINT*10^$NR_DECIMALS_CHAIN_SPECIFIC/1" | bc)
    drtpy contract call ${ALICE_ADDRESS} --recall-nonce "${DRTPY_SIGN[@]}" \
    --gas-limit=300000 --function="DCDTLocalMint" \
    --arguments str:${CHAIN_SPECIFIC_TOKEN} ${VALUE_TO_MINT} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}