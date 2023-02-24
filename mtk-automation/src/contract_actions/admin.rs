//! Module Admin
//!
//! Module to obtain and modify the admin_id

use crate::contract_actions::datakey::DataKey;
use soroban_sdk::{Address, Env};

pub(crate) fn get_admin_id(env: &Env) -> Address {
    let key = DataKey::AdminId;
    env.storage().get(&key).unwrap().unwrap()
}

pub(crate) fn set_admin_id(env: &Env, account_id: &Address) {
    env.storage().set(&DataKey::AdminId, account_id);
}

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::AdminId;
    e.storage().has(&key)
}

pub fn check_admin(e: &Env, admin: &Address) {
    if admin != &get_admin_id(e) {
        panic!("Not authorized by admin")
    }
}
