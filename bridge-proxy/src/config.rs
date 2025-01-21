use dharitri_sc::imports::*;

use transaction::EthTransaction;

#[dharitri_sc::module]
pub trait ConfigModule {
    #[only_owner]
    #[endpoint(setMultiTransferAddress)]
    fn set_multi_transfer_contract_address(
        &self,
        opt_multi_transfer_address: OptionalValue<ManagedAddress>,
    ) {
        match opt_multi_transfer_address {
            OptionalValue::Some(sc_addr) => {
                require!(
                    self.blockchain().is_smart_contract(&sc_addr),
                    "Invalid multi-transfer address"
                );
                self.multi_transfer_address().set(&sc_addr);
            }
            OptionalValue::None => self.multi_transfer_address().clear(),
        }
    }

    #[only_owner]
    #[endpoint(setBridgedTokensWrapperAddress)]
    fn set_bridged_tokens_wrapper_contract_address(
        &self,
        opt_address: OptionalValue<ManagedAddress>,
    ) {
        match opt_address {
            OptionalValue::Some(sc_addr) => {
                require!(
                    self.blockchain().is_smart_contract(&sc_addr),
                    "Invalid bridged tokens wrapper address"
                );
                self.bridged_tokens_wrapper_address().set(&sc_addr);
            }
            OptionalValue::None => self.bridged_tokens_wrapper_address().clear(),
        }
    }

    #[only_owner]
    #[endpoint(setDcdtSafeAddress)]
    fn set_dcdt_safe_contract_address(&self, opt_address: OptionalValue<ManagedAddress>) {
        match opt_address {
            OptionalValue::Some(sc_addr) => {
                require!(
                    self.blockchain().is_smart_contract(&sc_addr),
                    "Invalid bridged tokens wrapper address"
                );
                self.dcdt_safe_contract_address().set(&sc_addr);
            }
            OptionalValue::None => self.dcdt_safe_contract_address().clear(),
        }
    }

    #[view(getMultiTransferAddress)]
    #[storage_mapper("multiTransferAddress")]
    fn multi_transfer_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getBridgedTokensWrapperAddress)]
    #[storage_mapper("bridgedTokensWrapperAddress")]
    fn bridged_tokens_wrapper_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getDcdtSafeContractAddress)]
    #[storage_mapper("dcdtSafeContractAddress")]
    fn dcdt_safe_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("pending_transactions")]
    fn pending_transactions(&self) -> MapMapper<usize, EthTransaction<Self::Api>>;

    #[storage_mapper("payments")]
    fn payments(&self, tx_id: usize) -> SingleValueMapper<DcdtTokenPayment<Self::Api>>;

    #[storage_mapper("batch_id")]
    fn batch_id(&self, tx_id: usize) -> SingleValueMapper<u64>;

    #[view(highestTxId)]
    #[storage_mapper("highest_tx_id")]
    fn highest_tx_id(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("ongoingExecution")]
    fn ongoing_execution(&self, tx_id: usize) -> SingleValueMapper<u64>;
}
