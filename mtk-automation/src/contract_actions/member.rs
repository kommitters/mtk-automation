//! Module Member
//!
//! This module lets you get and revoke members of the organization
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::token_operation;
use soroban_sdk::{vec, Address, Env, RawVal, Vec};

pub fn add_member(env: &Env, account: &Address) {
    let mut members = get_members(env);
    members.push_back(account.clone());
    let key: DataKey = DataKey::Members;
    env.storage().set(&key, &members);
}

pub fn revoke_membership(env: &Env, from: &Address) {
    let mut members: Vec<Address> = get_members(env);

    let index;

    match members.first_index_of(from) {
        Some(i) => index = i,
        None => {
            panic!("You are trying to remove an account that doesn't belong to your organization")
        }
    }

    from.require_auth();
    members.remove(index);
    let key: DataKey = DataKey::Members;
    env.storage().set(&key, &members);

    token_operation::bring_back_tokens_to_admin(env, from)
}

pub fn get_members<T: soroban_sdk::TryFromVal<Env, RawVal> + soroban_sdk::IntoVal<Env, RawVal>>(
    e: &Env,
) -> Vec<T> {
    let key: DataKey = DataKey::Members;
    e.storage()
        .get(&key)
        .unwrap_or(Ok(vec![e])) // if no members on vector
        .unwrap()
}

pub fn is_member(env: &Env, to: &Address) -> bool {
    let members: Vec<Address> = get_members(env);
    members.contains(to)
}
