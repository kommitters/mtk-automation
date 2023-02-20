//! Module Admin
//!
//! Module to obtain and modify the admin_id

use crate::contract_actions::datakey::DataKey;
use soroban_sdk::{Address, Env};

pub fn get_admin_id(env: &Env) -> Address {
    let key = DataKey::AdminId;
    env.storage().get(&key).unwrap().unwrap()
}

pub fn set_admin_id(env: &Env, account_id: &Address) {
    env.storage().set(&DataKey::AdminId, account_id);
}
