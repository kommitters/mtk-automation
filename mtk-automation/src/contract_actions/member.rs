//! Module Member
//!
//! This module lets you get and revoke members of the organization

use crate::contract_actions::admin;
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::identifier_wrapper as identifier;
use crate::contract_actions::token;
use crate::contract_actions::token_contract;
use soroban_auth::Signature;
use soroban_sdk::{vec, AccountId, Env, RawVal, Vec};

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

    bring_back_tokens_to_admin(&env, &from)
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

fn bring_back_tokens_to_admin(env: &Env, from: &AccountId) {
    let tc_id = token_contract::get_token_contract_id(&env);
    let client = token::Client::new(&env, &tc_id);

    let admin_id = admin::get_admin_id(&env);
    let from_identifier = identifier::get_account_identifier(from.clone());
    let member_balance = client.balance(&from_identifier);

    swap_token_to();

    client.xfer_from(
        &Signature::Invoker,
        &0,
        &from_identifier,
        &admin_id,
        &member_balance,
    );
}

fn swap_token_to() {
    todo!("Do the swapping process from MKT to other aviable token");
}
