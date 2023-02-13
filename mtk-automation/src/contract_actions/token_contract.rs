// Module for defining and obtaining the contract token
pub mod token_contract {
    use soroban_sdk::{Env, BytesN};
    use crate::contract_actions::datakey::DataKey;

    pub fn get_token_contract_id(env: &Env) -> BytesN<32> {
        let key = DataKey::TokenId;
        env.storage().get(key).unwrap().unwrap()
    }

    pub fn set_token_id(e: &Env, token_id: &BytesN<32>) {
        e.storage().set(DataKey::TokenId, token_id);
    }
}
