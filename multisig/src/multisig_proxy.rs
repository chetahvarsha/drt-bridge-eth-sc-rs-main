// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct MultisigProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for MultisigProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = MultisigProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        MultisigProxyMethods { wrapped_tx: tx }
    }
}

pub struct MultisigProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> MultisigProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    /// DcdtSafe and MultiTransferDcdt are expected to be deployed and configured separately, 
    /// and then having their ownership changed to this Multisig SC. 
    pub fn init<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<ManagedAddress<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<BigUint<Env::Api>>,
        Arg5: ProxyArg<usize>,
        Arg6: ProxyArg<MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>>,
    >(
        self,
        dcdt_safe_sc_address: Arg0,
        multi_transfer_sc_address: Arg1,
        proxy_sc_address: Arg2,
        required_stake: Arg3,
        slash_amount: Arg4,
        quorum: Arg5,
        board: Arg6,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&dcdt_safe_sc_address)
            .argument(&multi_transfer_sc_address)
            .argument(&proxy_sc_address)
            .argument(&required_stake)
            .argument(&slash_amount)
            .argument(&quorum)
            .argument(&board)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> MultisigProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        dcdt_safe_sc_address: Arg0,
        multi_transfer_sc_address: Arg1,
        proxy_sc_address: Arg2,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .argument(&dcdt_safe_sc_address)
            .argument(&multi_transfer_sc_address)
            .argument(&proxy_sc_address)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> MultisigProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// Distributes the accumulated fees to the given addresses. 
    /// Expected arguments are pairs of (address, percentage), 
    /// where percentages must add up to the PERCENTAGE_TOTAL constant 
    pub fn distribute_fees_from_child_contracts<
        Arg0: ProxyArg<MultiValueEncoded<Env::Api, MultiValue2<ManagedAddress<Env::Api>, u32>>>,
    >(
        self,
        dest_address_percentage_pairs: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("distributeFeesFromChildContracts")
            .argument(&dest_address_percentage_pairs)
            .original_result()
    }

    /// Board members have to stake a certain amount of REWA 
    /// before being allowed to sign actions 
    pub fn stake(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("stake")
            .original_result()
    }

    pub fn unstake<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        amount: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unstake")
            .argument(&amount)
            .original_result()
    }

    /// After a batch is processed on the Ethereum side, 
    /// the DcdtSafe expects a list of statuses of said transactions (success or failure). 
    ///  
    /// This endpoint proposes an action to set the statuses to a certain list of values. 
    /// Nothing is changed in the DcdtSafe contract until the action is signed and executed. 
    pub fn propose_dcdt_safe_set_current_transaction_batch_status<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, transaction::transaction_status::TransactionStatus>>,
    >(
        self,
        dcdt_safe_batch_id: Arg0,
        tx_batch_status: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("proposeDcdtSafeSetCurrentTransactionBatchStatus")
            .argument(&dcdt_safe_batch_id)
            .argument(&tx_batch_status)
            .original_result()
    }

    /// Proposes a batch of Ethereum -> Dharitri transfers. 
    /// Transactions have to be separated by fields, in the following order: 
    /// Sender Address, Destination Address, Token ID, Amount, Tx Nonce 
    pub fn propose_multi_transfer_dcdt_batch<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue6<eth_address::EthAddress<Env::Api>, ManagedAddress<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>, u64, ManagedOption<Env::Api, ManagedBuffer<Env::Api>>>>>,
    >(
        self,
        eth_batch_id: Arg0,
        transfers: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("proposeMultiTransferDcdtBatch")
            .argument(&eth_batch_id)
            .argument(&transfers)
            .original_result()
    }

    /// Failed Ethereum -> Dharitri transactions are saved in the MultiTransfer SC 
    /// as "refund transactions", and stored in batches, using the same mechanism as DcdtSafe. 
    ///  
    /// This function moves the first refund batch into the DcdtSafe SC, 
    /// converting the transactions into Dharitri -> Ethereum transactions 
    /// and adding them into DcdtSafe batches 
    pub fn move_refund_batch_to_safe_from_child_contract(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("moveRefundBatchToSafeFromChildContract")
            .original_result()
    }

    pub fn init_supply_from_child_contract<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("initSupplyFromChildContract")
            .argument(&token_id)
            .argument(&amount)
            .original_result()
    }

    pub fn add_unprocessed_refund_tx_to_batch<
        Arg0: ProxyArg<u64>,
    >(
        self,
        tx_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addUnprocessedRefundTxToBatch")
            .argument(&tx_id)
            .original_result()
    }

    pub fn withdraw_refund_fees_for_ethereum<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdrawRefundFeesForEthereum")
            .argument(&token_id)
            .original_result()
    }

    pub fn withdraw_transaction_fees<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdrawTransactionFees")
            .argument(&token_id)
            .original_result()
    }

    pub fn withdraw_slashed_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("withdrawSlashedAmount")
            .original_result()
    }

    /// Proposers and board members use this to launch signed actions. 
    pub fn perform_action_endpoint<
        Arg0: ProxyArg<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("performAction")
            .argument(&action_id)
            .original_result()
    }

    /// Used by board members to sign actions. 
    pub fn sign<
        Arg0: ProxyArg<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("sign")
            .argument(&action_id)
            .original_result()
    }

    pub fn upgrade_child_contract_from_source<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<bool>,
        Arg3: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        child_sc_address: Arg0,
        source_address: Arg1,
        is_payable: Arg2,
        init_args: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("upgradeChildContractFromSource")
            .argument(&child_sc_address)
            .argument(&source_address)
            .argument(&is_payable)
            .argument(&init_args)
            .original_result()
    }

    pub fn add_board_member_endpoint<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        board_member: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addBoardMember")
            .argument(&board_member)
            .original_result()
    }

    pub fn remove_user<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        board_member: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("removeUser")
            .argument(&board_member)
            .original_result()
    }

    /// Cuts a fixed amount from a board member's stake. 
    /// This should be used only in cases where the board member 
    /// is being actively malicious. 
    ///  
    /// After stake is cut, the board member would have to stake again 
    /// to be able to sign actions. 
    pub fn slash_board_member<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        board_member: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("slashBoardMember")
            .argument(&board_member)
            .original_result()
    }

    pub fn change_quorum<
        Arg0: ProxyArg<usize>,
    >(
        self,
        new_quorum: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeQuorum")
            .argument(&new_quorum)
            .original_result()
    }

    /// Maps an DCDT token to an ERC20 address. Used by relayers. 
    pub fn add_mapping<
        Arg0: ProxyArg<eth_address::EthAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        erc20_address: Arg0,
        token_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("addMapping")
            .argument(&erc20_address)
            .argument(&token_id)
            .original_result()
    }

    pub fn clear_mapping<
        Arg0: ProxyArg<eth_address::EthAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        erc20_address: Arg0,
        token_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("clearMapping")
            .argument(&erc20_address)
            .argument(&token_id)
            .original_result()
    }

    pub fn pause_dcdt_safe(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pauseDcdtSafe")
            .original_result()
    }

    pub fn unpause_dcdt_safe(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unpauseDcdtSafe")
            .original_result()
    }

    pub fn init_supply_dcdt_safe<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("initSupplyDcdtSafe")
            .argument(&token_id)
            .argument(&amount)
            .original_result()
    }

    pub fn init_supply_mint_burn_dcdt_safe<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        mint_amount: Arg1,
        burn_amount: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("initSupplyMintBurnDcdtSafe")
            .argument(&token_id)
            .argument(&mint_amount)
            .argument(&burn_amount)
            .original_result()
    }

    pub fn pause_proxy(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pauseProxy")
            .original_result()
    }

    pub fn unpause_proxy(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unpauseProxy")
            .original_result()
    }

    pub fn change_fee_estimator_contract_address<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        new_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeFeeEstimatorContractAddress")
            .argument(&new_address)
            .original_result()
    }

    /// Sets the gas limit being used for Ethereum transactions 
    /// This is used in the DcdtSafe contract to determine the fee amount 
    ///  
    /// fee_amount = eth_gas_limit * price_per_gas_unit 
    ///  
    /// where price_per_gas_unit is queried from the aggregator (fee estimator SC) 
    pub fn change_dharitri_to_eth_gas_limit<
        Arg0: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        new_gas_limit: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeDharitriToEthGasLimit")
            .argument(&new_gas_limit)
            .original_result()
    }

    /// Default price being used if the aggregator lacks a mapping for this token 
    /// or the aggregator address is not set 
    pub fn change_default_price_per_gas_unit<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        new_value: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeDefaultPricePerGasUnit")
            .argument(&token_id)
            .argument(&new_value)
            .original_result()
    }

    /// Token ticker being used when querying the aggregator for GWEI prices 
    pub fn change_token_ticker<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        new_ticker: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeTokenTicker")
            .argument(&token_id)
            .argument(&new_ticker)
            .original_result()
    }

    pub fn dcdt_safe_add_token_to_whitelist<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<bool>,
        Arg3: ProxyArg<bool>,
        Arg4: ProxyArg<BigUint<Env::Api>>,
        Arg5: ProxyArg<BigUint<Env::Api>>,
        Arg6: ProxyArg<BigUint<Env::Api>>,
        Arg7: ProxyArg<OptionalValue<BigUint<Env::Api>>>,
    >(
        self,
        token_id: Arg0,
        ticker: Arg1,
        mint_burn_allowed: Arg2,
        is_native_token: Arg3,
        total_balance: Arg4,
        mint_balance: Arg5,
        burn_balance: Arg6,
        opt_default_price_per_gas_unit: Arg7,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdtSafeAddTokenToWhitelist")
            .argument(&token_id)
            .argument(&ticker)
            .argument(&mint_burn_allowed)
            .argument(&is_native_token)
            .argument(&total_balance)
            .argument(&mint_balance)
            .argument(&burn_balance)
            .argument(&opt_default_price_per_gas_unit)
            .original_result()
    }

    pub fn set_multi_transfer_on_dcdt_safe(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setMultiTransferOnDcdtSafe")
            .original_result()
    }

    pub fn set_dcdt_safe_on_multi_transfer(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setDcdtSafeOnMultiTransfer")
            .original_result()
    }

    pub fn dcdt_safe_remove_token_from_whitelist<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdtSafeRemoveTokenFromWhitelist")
            .argument(&token_id)
            .original_result()
    }

    /// Sets maximum batch size for the DcdtSafe SC. 
    /// If a batch reaches this amount of transactions, it is considered full, 
    /// and a new incoming transaction will be put into a new batch. 
    pub fn dcdt_safe_set_max_tx_batch_size<
        Arg0: ProxyArg<usize>,
    >(
        self,
        new_max_tx_batch_size: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdtSafeSetMaxTxBatchSize")
            .argument(&new_max_tx_batch_size)
            .original_result()
    }

    /// Sets the maximum block duration in which an DcdtSafe batch accepts transactions 
    /// For a batch to be considered "full", it has to either reach `maxTxBatchSize` transactions, 
    /// or have txBatchBlockDuration blocks pass since the first tx was added in the batch 
    pub fn dcdt_safe_set_max_tx_batch_block_duration<
        Arg0: ProxyArg<u64>,
    >(
        self,
        new_max_tx_batch_block_duration: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdtSafeSetMaxTxBatchBlockDuration")
            .argument(&new_max_tx_batch_block_duration)
            .original_result()
    }

    /// Sets the maximum bridged amount for the token for the Dharitri -> Ethereum direction. 
    /// Any attempt to transfer over this amount will be rejected. 
    pub fn dcdt_safe_set_max_bridged_amount_for_token<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        max_amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("dcdtSafeSetMaxBridgedAmountForToken")
            .argument(&token_id)
            .argument(&max_amount)
            .original_result()
    }

    /// Same as the function above, but for Ethereum -> Dharitri transactions. 
    pub fn multi_transfer_dcdt_set_max_bridged_amount_for_token<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_id: Arg0,
        max_amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multiTransferDcdtSetMaxBridgedAmountForToken")
            .argument(&token_id)
            .argument(&max_amount)
            .original_result()
    }

    /// Any failed Ethereum -> Dharitri transactions are added into so-called "refund batches\ 
    /// This configures the size of a batch. 
    pub fn multi_transfer_dcdt_set_max_refund_tx_batch_size<
        Arg0: ProxyArg<usize>,
    >(
        self,
        new_max_tx_batch_size: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multiTransferDcdtSetMaxRefundTxBatchSize")
            .argument(&new_max_tx_batch_size)
            .original_result()
    }

    /// Max block duration for refund batches. Default is "infinite" (u64::MAX) 
    /// and only max batch size matters 
    pub fn multi_transfer_dcdt_set_max_refund_tx_batch_block_duration<
        Arg0: ProxyArg<u64>,
    >(
        self,
        new_max_tx_batch_block_duration: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multiTransferDcdtSetMaxRefundTxBatchBlockDuration")
            .argument(&new_max_tx_batch_block_duration)
            .original_result()
    }

    /// Sets the wrapping contract address. 
    /// This contract is used to map multiple tokens to a universal one. 
    /// Useful in cases where a single token (USDC for example) 
    /// is being transferred from multiple chains. 
    ///  
    /// They will all have different token IDs, but can be swapped 1:1 in the wrapping SC. 
    /// The wrapping is done automatically, so the user only receives the universal token. 
    pub fn multi_transfer_dcdt_set_wrapping_contract_address<
        Arg0: ProxyArg<OptionalValue<ManagedAddress<Env::Api>>>,
    >(
        self,
        opt_wrapping_contract_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("multiTransferDcdtSetWrappingContractAddress")
            .argument(&opt_wrapping_contract_address)
            .original_result()
    }

    /// Minimum number of signatures needed to perform any action. 
    pub fn quorum(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getQuorum")
            .original_result()
    }

    /// Denormalized board member count. 
    /// It is kept in sync with the user list by the contract. 
    pub fn num_board_members(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNumBoardMembers")
            .original_result()
    }

    /// The required amount to stake for accepting relayer position 
    pub fn required_stake_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRequiredStakeAmount")
            .original_result()
    }

    /// Staked amount by each board member. 
    pub fn amount_staked<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        board_member_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAmountStaked")
            .argument(&board_member_address)
            .original_result()
    }

    /// Amount of stake slashed if a relayer is misbehaving 
    pub fn slash_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getSlashAmount")
            .original_result()
    }

    /// Total slashed tokens accumulated 
    pub fn slashed_tokens_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getSlashedTokensAmount")
            .original_result()
    }

    pub fn last_executed_eth_batch_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLastExecutedEthBatchId")
            .original_result()
    }

    pub fn last_executed_eth_tx_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLastExecutedEthTxId")
            .original_result()
    }

    /// Mapping between ERC20 Ethereum address and Dharitri DCDT Token Identifiers 
    pub fn erc20_address_for_token_id<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, eth_address::EthAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getErc20AddressForTokenId")
            .argument(&token_id)
            .original_result()
    }

    pub fn token_id_for_erc20_address<
        Arg0: ProxyArg<eth_address::EthAddress<Env::Api>>,
    >(
        self,
        erc20_address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTokenIdForErc20Address")
            .argument(&erc20_address)
            .original_result()
    }

    pub fn dcdt_safe_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDcdtSafeAddress")
            .original_result()
    }

    pub fn multi_transfer_dcdt_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getMultiTransferDcdtAddress")
            .original_result()
    }

    pub fn proxy_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getProxyAddress")
            .original_result()
    }

    /// Returns the current DcdtSafe batch. 
    ///  
    /// First result is the batch ID, then pairs of 6 results, representing transactions 
    /// split by fields: 
    ///  
    /// Block Nonce, Tx Nonce, Sender Address, Receiver Address, Token ID, Amount 
    pub fn get_current_tx_batch(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue2<u64, MultiValueEncoded<Env::Api, MultiValue6<u64, u64, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>>>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCurrentTxBatch")
            .original_result()
    }

    /// Returns the DcdtSafe batch that has the provided batch_id. 
    ///  
    /// First result is the batch ID, then pairs of 6 results, representing transactions 
    /// split by fields: 
    ///  
    /// Block Nonce, Tx Nonce, Sender Address, Receiver Address, Token ID, Amount 
    pub fn get_batch<
        Arg0: ProxyArg<u64>,
    >(
        self,
        batch_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue2<u64, MultiValueEncoded<Env::Api, MultiValue6<u64, u64, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>>>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBatch")
            .argument(&batch_id)
            .original_result()
    }

    /// Returns a batch of failed Ethereum -> Dharitri transactions. 
    /// The result format is the same as getCurrentTxBatch 
    pub fn get_current_refund_batch(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<MultiValue2<u64, MultiValueEncoded<Env::Api, MultiValue6<u64, u64, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>>>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCurrentRefundBatch")
            .original_result()
    }

    /// Actions are cleared after execution, so an empty entry means the action was executed already 
    /// Returns "false" if the action ID is invalid 
    pub fn was_action_executed<
        Arg0: ProxyArg<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("wasActionExecuted")
            .argument(&action_id)
            .original_result()
    }

    /// Used for Ethereum -> Dharitri batches. 
    /// If the mapping was made, it means that the transfer action was proposed in the past. 
    /// To check if it was executed as well, use the wasActionExecuted view 
    pub fn was_transfer_action_proposed<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue6<eth_address::EthAddress<Env::Api>, ManagedAddress<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>, u64, ManagedOption<Env::Api, ManagedBuffer<Env::Api>>>>>,
    >(
        self,
        eth_batch_id: Arg0,
        transfers: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("wasTransferActionProposed")
            .argument(&eth_batch_id)
            .argument(&transfers)
            .original_result()
    }

    /// Used for Ethereum -> Dharitri batches. 
    /// If `wasActionExecuted` returns true, then this can be used to get the action ID. 
    /// Will return 0 if the transfers were not proposed 
    pub fn get_action_id_for_transfer_batch<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue6<eth_address::EthAddress<Env::Api>, ManagedAddress<Env::Api>, TokenIdentifier<Env::Api>, BigUint<Env::Api>, u64, ManagedOption<Env::Api, ManagedBuffer<Env::Api>>>>>,
    >(
        self,
        eth_batch_id: Arg0,
        transfers: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getActionIdForTransferBatch")
            .argument(&eth_batch_id)
            .argument(&transfers)
            .original_result()
    }

    /// Used for Dharitri -> Ethereum batches. 
    /// Returns "true" if an action was already proposed for the given batch, 
    /// with these exact transaction statuses, in this exact order 
    pub fn was_set_current_transaction_batch_status_action_proposed<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, transaction::transaction_status::TransactionStatus>>,
    >(
        self,
        dcdt_safe_batch_id: Arg0,
        expected_tx_batch_status: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("wasSetCurrentTransactionBatchStatusActionProposed")
            .argument(&dcdt_safe_batch_id)
            .argument(&expected_tx_batch_status)
            .original_result()
    }

    /// If `wasSetCurrentTransactionBatchStatusActionProposed` return true, 
    /// this can be used to get the action ID. 
    /// Will return 0 if the set status action was not proposed 
    pub fn get_action_id_for_set_current_transaction_batch_status<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, transaction::transaction_status::TransactionStatus>>,
    >(
        self,
        dcdt_safe_batch_id: Arg0,
        expected_tx_batch_status: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getActionIdForSetCurrentTransactionBatchStatus")
            .argument(&dcdt_safe_batch_id)
            .argument(&expected_tx_batch_status)
            .original_result()
    }

    /// Returns `true` (`1`) if the user has signed the action. 
    /// Does not check whether or not the user is still a board member and the signature valid. 
    pub fn signed<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<usize>,
    >(
        self,
        user: Arg0,
        action_id: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("signed")
            .argument(&user)
            .argument(&action_id)
            .original_result()
    }

    /// Indicates user rights. 
    /// `0` = no rights, 
    /// `1` = can propose. Can also sign if they have enough stake. 
    pub fn user_role<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        user: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, UserRole> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("userRole")
            .argument(&user)
            .original_result()
    }

    /// Lists all board members 
    pub fn get_all_board_members(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllBoardMembers")
            .original_result()
    }

    /// Lists all board members that staked the correct amount. 
    /// A board member with not enough stake can propose, but cannot sign. 
    pub fn get_all_staked_relayers(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAllStakedRelayers")
            .original_result()
    }

    /// Gets the number of signatures for the action with the given ID 
    pub fn get_action_signer_count<
        Arg0: ProxyArg<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getActionSignerCount")
            .argument(&action_id)
            .original_result()
    }

    /// It is possible for board members to lose their role. 
    /// They are not automatically removed from all actions when doing so, 
    /// therefore the contract needs to re-check every time when actions are performed. 
    /// This function is used to validate the signers before performing an action. 
    /// It also makes it easy to check before performing an action. 
    pub fn get_action_valid_signer_count<
        Arg0: ProxyArg<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getActionValidSignerCount")
            .argument(&action_id)
            .original_result()
    }

    /// Returns `true` (`1`) if `getActionValidSignerCount >= getQuorum`. 
    pub fn quorum_reached<
        Arg0: ProxyArg<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("quorumReached")
            .argument(&action_id)
            .original_result()
    }

    /// The index of the last proposed action. 
    /// 0 means that no action was ever proposed yet. 
    pub fn get_action_last_index(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, usize> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getActionLastIndex")
            .original_result()
    }

    /// Serialized action data of an action with index. 
    pub fn get_action_data<
        Arg0: ProxyArg<usize>,
    >(
        self,
        action_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, Action<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getActionData")
            .argument(&action_id)
            .original_result()
    }

    pub fn pause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pause")
            .original_result()
    }

    pub fn unpause_endpoint(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unpause")
            .original_result()
    }

    pub fn paused_status(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("isPaused")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode, Clone, Copy, PartialEq)]
pub enum UserRole {
    None,
    BoardMember,
}

#[rustfmt::skip]
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub enum Action<Api>
where
    Api: ManagedTypeApi,
{
    Nothing,
    SetCurrentTransactionBatchStatus {
        dcdt_safe_batch_id: u64,
        tx_batch_status: ManagedVec<Api, transaction::transaction_status::TransactionStatus>,
    },
    BatchTransferDcdtToken {
        eth_batch_id: u64,
        transfers: ManagedVec<Api, transaction::EthTransaction<Api>>,
    },
}
