//! Module Exchange_Contract
//!
//! Module that allows to store and request the Exchange contract Id
use crate::contract_actions::datakey::DataKey;
use soroban_sdk::{BytesN, Env};

pub(crate) fn get_exchange_contract_id(env: &Env) -> BytesN<32> {
    let key = DataKey::ExId;
    env.storage().get(&key).unwrap().unwrap()
}

pub(crate) fn set_exchange_contract_id(env: &Env, contract_id: &BytesN<32>) {
    env.storage().set(&DataKey::ExId, contract_id);
}
