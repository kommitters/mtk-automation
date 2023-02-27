//! Offset Module
//!
//! Contract's offset control module
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::member;
use crate::contract_actions::token_operation;
use soroban_sdk::{Address, Env, Map, Symbol};

pub(crate) fn set_offset(env: &Env, offset_types: &Map<Symbol, i32>) {
    env.storage().set(&DataKey::Offsets, offset_types);
}

pub(crate) fn offset_a_member(
    env: &Env,
    admin_address: &Address,
    to: &Address,
    offset_type: &Symbol,
) {
    if !member::is_member(env, to) {
        panic!("The user account you're trying to offset doesn't belong to the organization");
    }

    if !is_offset_valid(env, offset_type) {
        panic!("The offset type you are trying to use isn't supported")
    }
    let offset_value = get_offset_by_type(env, offset_type) as i128;
    token_operation::transfer(env, admin_address, &to.clone(), &offset_value);
}

fn get_offset_by_type(env: &Env, o_type: &Symbol) -> i32 {
    let key = DataKey::Offsets;
    let offsets: Map<Symbol, i32> = env.storage().get(&key).unwrap().unwrap();

    offsets.get(*o_type).unwrap().unwrap()
}

fn get_offset(env: &Env) -> Map<Symbol, i32> {
    let key = DataKey::Offsets;
    env.storage().get(&key).unwrap().unwrap()
}

fn is_offset_valid(env: &Env, key: &Symbol) -> bool {
    let offset = get_offset(env);
    offset.contains_key(*key)
}
