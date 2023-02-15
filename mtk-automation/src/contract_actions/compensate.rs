//! Compensate Module
//!
//! Module for compensates control in the contract
use soroban_sdk::{Env, Symbol, Map, AccountId};
use soroban_auth::Signature;
use crate::contract_actions::member;
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::identifier_wrapper;
use crate::contract_actions::token_operation;

pub fn set_compensations(env: &Env, compensate_types: &Map<Symbol, u32>) {
    env.storage().set(DataKey::Compensate, compensate_types);
}

pub fn compensate_member(env: &Env, approval_sign: &Signature, to: &AccountId, compensate_types: &Symbol) {
    if !member::is_member(&env, &to) {
        panic!("The user account you're trying to compensate doesn't belong to the organization");
    }

    if !is_compensation_valid(&env, &compensate_types) {
        panic!("The compensate type you are trying to use isn't supported")
    }

    let compensate_value = get_compensation_by_type(&env, &compensate_types);
    token_operation::transfer(&env, &approval_sign, &identifier_wrapper::get_account_identifier(to.clone()), &compensate_value);
}

fn get_compensation_by_type(env: &Env, r_type: &Symbol) -> i128 {
    let key = DataKey::Compensate;
    let compensate: Map<Symbol, i128> = env.storage().get(key).unwrap().unwrap();

    compensate.get(r_type.clone()).unwrap().unwrap()
}

fn get_compensations(env: &Env) -> Map<Symbol, u32> {
    let key = DataKey::Compensate;
    env.storage().get(key).unwrap().unwrap()
}

fn is_compensation_valid(env: &Env, key: &Symbol) -> bool {
    let compensates = get_compensations(&env);
    compensates.contains_key(key.clone())
}
