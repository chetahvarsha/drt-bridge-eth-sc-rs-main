use dharitri_sc::imports::*;

use transaction::{EthTransaction, EthTxAsMultiValue};

use crate::storage::EthBatchHash;
use crate::user_role::UserRole;

#[dharitri_sc::module]
pub trait UtilModule: crate::storage::StorageModule {
    fn get_user_role(&self, user: &ManagedAddress) -> UserRole {
        let user_id = self.user_mapper().get_user_id(user);
        if user_id == 0 {
            UserRole::None
        } else {
            self.user_id_to_role(user_id).get()
        }
    }

    fn is_valid_action_id(&self, action_id: usize) -> bool {
        let min_id = 1;
        let max_id = self.action_mapper().len();

        action_id >= min_id && action_id <= max_id
    }

    fn get_all_users_with_role(&self, role: UserRole) -> MultiValueEncoded<ManagedAddress> {
        let mut result = ManagedVec::new();
        let num_users = self.user_mapper().get_user_count();
        for user_id in 1..=num_users {
            if self.user_id_to_role(user_id).get() == role {
                if let Some(address) = self.user_mapper().get_user_address(user_id) {
                    result.push(address);
                }
            }
        }
        result.into()
    }

    fn has_enough_stake(&self, board_member_address: &ManagedAddress) -> bool {
        let required_stake = self.required_stake_amount().get();
        let amount_staked = self.amount_staked(board_member_address).get();

        amount_staked >= required_stake
    }

    fn transfers_multi_value_to_eth_tx_vec(
        &self,
        transfers: MultiValueEncoded<EthTxAsMultiValue<Self::Api>>,
    ) -> ManagedVec<EthTransaction<Self::Api>> {
        let mut transfers_as_eth_tx = ManagedVec::new();
        for transfer in transfers {
            let (from, to, token_id, amount, tx_nonce, call_data) = transfer.into_tuple();

            transfers_as_eth_tx.push(EthTransaction {
                from,
                to,
                token_id,
                amount,
                tx_nonce,
                call_data,
            });
        }

        transfers_as_eth_tx
    }

    fn require_valid_eth_tx_ids(&self, eth_tx_vec: &ManagedVec<EthTransaction<Self::Api>>) {
        let last_executed_eth_tx_id = self.last_executed_eth_tx_id().get();
        let mut current_expected_tx_id = last_executed_eth_tx_id + 1;

        for eth_tx in eth_tx_vec {
            require!(eth_tx.tx_nonce == current_expected_tx_id, "Invalid Tx ID");
            current_expected_tx_id += 1;
        }
    }

    fn hash_eth_tx_batch(
        &self,
        eth_tx_batch: &ManagedVec<EthTransaction<Self::Api>>,
    ) -> EthBatchHash<Self::Api> {
        let mut serialized = ManagedBuffer::new();
        if eth_tx_batch.top_encode(&mut serialized).is_err() {
            sc_panic!("Failed to serialized batch");
        }

        self.crypto().keccak256(&serialized)
    }
}
