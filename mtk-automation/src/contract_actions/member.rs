//! Module Member
//!
//! This module lets you get and revoke members of the organization

use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::token_operation;
use soroban_sdk::{symbol, vec, Address, Env, RawVal, Symbol, Vec};

pub(crate) fn add_member(env: &Env, member: Address, admin: Address) {
    admin.require_auth();
    if is_member(env, &member) {
        panic!("Member Already exists")
    };
    let mut members = get_members(env);
    members.push_back(member);
    let key: DataKey = DataKey::Members;
    env.storage().set(&key, &members);
}

pub(crate) fn revoke_membership1(env: &Env, from: &Address) {
    let members: Vec<Address> = get_members(env);
    find_if_revocable(from, &members).unwrap();
    token_operation::bring_back_tokens_to_admin1(env, from)
}

pub(crate) fn revoke_membership2(env: &Env, from: &Address) {
    let members: Vec<Address> = get_members(env);
    find_if_revocable(from, &members).unwrap();
    token_operation::bring_back_tokens_to_admin2(env, from)
}

pub(crate) fn revoke_membership3(env: &Env, from: &Address) {
    let mut members: Vec<Address> = get_members(env);
    let index = find_if_revocable(from, &members).unwrap();

    members.remove(index);
    let key: DataKey = DataKey::Members;
    env.storage().set(&key, &members);

    token_operation::bring_back_tokens_to_admin3(env)
}

pub(crate) fn get_members<
    T: soroban_sdk::TryFromVal<Env, RawVal> + soroban_sdk::IntoVal<Env, RawVal>,
>(
    e: &Env,
) -> Vec<T> {
    let key: DataKey = DataKey::Members;
    e.storage()
        .get(&key)
        .unwrap_or(Ok(vec![e])) // if no members on vector
        .unwrap()
}

pub(crate) fn is_member(env: &Env, to: &Address) -> bool {
    let members: Vec<Address> = get_members(env);
    members.contains(to)
}

fn find_if_revocable(from: &Address, members: &Vec<Address>) -> Result<u32, Symbol> {
    match members.first_index_of(from) {
        Some(i) => Ok(i),
        None => Err(symbol!("error")),
    }
}
