//! Module Member
//!
//! This module lets you get and revoke members of the organization
use soroban_sdk::{vec, AccountId, Env, RawVal, Vec};
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::token_operation;

pub fn add_member(env: &Env, account: AccountId) {
    let mut members = get_members(&env);
    members.push_back(account);
    let key: DataKey = DataKey::Members;
    env.storage().set(key, members);
}

pub fn revoke_membership(env: &Env, from: &AccountId) {
    let mut members: Vec<AccountId> = get_members(&env);

    let index;

    match members.first_index_of(from) {
        Some(i) => index = i,
        None => {
            panic!("You are trying to remove an account that doesn't belong to your organization")
        }
    }

    members.remove(index);
    let key: DataKey = DataKey::Members;
    env.storage().set(key, members);

    token_operation::bring_back_tokens_to_admin(&env, &from)
}

pub fn get_members<T: soroban_sdk::TryFromVal<Env, RawVal> + soroban_sdk::IntoVal<Env, RawVal>>(
    e: &Env,
) -> Vec<T> {
    let key: DataKey = DataKey::Members;
    e.storage()
        .get(key)
        .unwrap_or(Ok(vec![e])) // if no members on vector
        .unwrap()
}
