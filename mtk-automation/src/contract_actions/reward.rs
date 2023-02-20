//! Reward Module
//!
//! Module for reward control in the contract
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::member;
use crate::contract_actions::token_operation;
use soroban_sdk::Address;
use soroban_sdk::{Env, Map, Symbol};

pub fn set_rewards(env: &Env, compensate_types: &Map<Symbol, u32>) {
    env.storage().set(&DataKey::Rewards, compensate_types);
}

pub fn reward_member(env: &Env, approval_sign: &Address, to: &Address, compensate_types: &Symbol) {
    if !member::is_member(env, to) {
        panic!("The user account you're trying to compensate doesn't belong to the organization");
    }

    if !is_reward_valid(env, compensate_types) {
        panic!("The compensate type you are trying to use isn't supported")
    }

    let compensate_value = get_compensation_by_type(env, compensate_types);
    token_operation::transfer(env, approval_sign, &to.clone(), &compensate_value);
}

fn get_compensation_by_type(env: &Env, r_type: &Symbol) -> i128 {
    let key = DataKey::Rewards;
    let compensate: Map<Symbol, i128> = env.storage().get(&key).unwrap().unwrap();

    compensate.get(*r_type).unwrap().unwrap()
}

fn get_rewards(env: &Env) -> Map<Symbol, u32> {
    let key = DataKey::Rewards;
    env.storage().get(&key).unwrap().unwrap()
}

fn is_reward_valid(env: &Env, key: &Symbol) -> bool {
    let compensates = get_rewards(env);
    compensates.contains_key(*key)
}
