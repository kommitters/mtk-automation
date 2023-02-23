//! Module Token_Contract
//!
//! Module for defining and obtaining the contract token
use crate::contract_actions::datakey::DataKey;
use soroban_sdk::{BytesN, Env};

pub(crate) fn get_token_contract_id(env: &Env) -> BytesN<32> {
    let key = DataKey::TokenId;
    env.storage().get(&key).unwrap().unwrap()
}

pub(crate) fn set_token_id(e: &Env, token_id: &BytesN<32>) {
    e.storage().set(&DataKey::TokenId, token_id);
}
