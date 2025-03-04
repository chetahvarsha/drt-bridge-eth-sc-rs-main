use dharitri_sc::derive_imports::*;

use dharitri_sc::{api::ManagedTypeApi, types::ManagedVec};
use transaction::{BlockNonce, TxNonce};

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub enum BatchStatus<M: ManagedTypeApi> {
    AlreadyProcessed,
    Empty,
    PartiallyFull {
        end_block_nonce: BlockNonce,
        tx_ids: ManagedVec<M, TxNonce>,
    },
    Full,
    WaitingForSignatures,
}
