#![no_std]

use dharitri_sc::derive_imports::*;
use dharitri_sc::imports::*;

use eth_address::EthAddress;
pub mod transaction_status;

// revert protection
pub const MIN_BLOCKS_FOR_FINALITY: u64 = 10;
pub const TX_MULTIRESULT_NR_FIELDS: usize = 6;

pub type TxNonce = u64;
pub type BlockNonce = u64;
pub type SenderAddressRaw<M> = ManagedBuffer<M>;
pub type ReceiverAddressRaw<M> = ManagedBuffer<M>;
pub type TxAsMultiValue<M> = MultiValue6<
    BlockNonce,
    TxNonce,
    SenderAddressRaw<M>,
    ReceiverAddressRaw<M>,
    TokenIdentifier<M>,
    BigUint<M>,
>;
pub type PaymentsVec<M> = ManagedVec<M, DcdtTokenPayment<M>>;
pub type TxBatchSplitInFields<M> = MultiValue2<u64, MultiValueEncoded<M, TxAsMultiValue<M>>>;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopDecode, TopEncode, Clone, ManagedVecItem)]
pub struct CallData<M: ManagedTypeApi> {
    pub endpoint: ManagedBuffer<M>,
    pub gas_limit: u64,
    pub args: ManagedOption<M, ManagedVec<M, ManagedBuffer<M>>>,
}

impl<M: ManagedTypeApi> Default for CallData<M> {
    fn default() -> Self {
        CallData {
            endpoint: ManagedBuffer::new(),
            gas_limit: 0u64,
            args: ManagedOption::none(),
        }
    }
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, ManagedVecItem)]
pub struct EthTransaction<M: ManagedTypeApi> {
    pub from: EthAddress<M>,
    pub to: ManagedAddress<M>,
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub tx_nonce: TxNonce,
    pub call_data: ManagedOption<M, ManagedBuffer<M>>,
}

pub type EthTxAsMultiValue<M> = MultiValue6<
    EthAddress<M>,
    ManagedAddress<M>,
    TokenIdentifier<M>,
    BigUint<M>,
    TxNonce,
    ManagedOption<M, ManagedBuffer<M>>,
>;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone)]
pub struct Transaction<M: ManagedTypeApi> {
    pub block_nonce: BlockNonce,
    pub nonce: TxNonce,
    pub from: ManagedBuffer<M>,
    pub to: ManagedBuffer<M>,
    pub token_identifier: TokenIdentifier<M>,
    pub amount: BigUint<M>,
    pub is_refund_tx: bool,
}

impl<M: ManagedTypeApi> From<TxAsMultiValue<M>> for Transaction<M> {
    fn from(tx_as_multiresult: TxAsMultiValue<M>) -> Self {
        let (block_nonce, nonce, from, to, token_identifier, amount) =
            tx_as_multiresult.into_tuple();

        Transaction {
            block_nonce,
            nonce,
            from,
            to,
            token_identifier,
            amount,
            is_refund_tx: false,
        }
    }
}

impl<M: ManagedTypeApi> Transaction<M> {
    pub fn into_multiresult(self) -> TxAsMultiValue<M> {
        (
            self.block_nonce,
            self.nonce,
            self.from,
            self.to,
            self.token_identifier,
            self.amount,
        )
            .into()
    }
}
