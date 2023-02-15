//! Reward Module
//!
//! Module for rewards control in the contract
use soroban_sdk::{Env, Symbol, Map, AccountId};
use soroban_auth::{Identifier, Signature};
use crate::contract_actions::validation;
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::identifier_wrapper;
use crate::contract_actions::token_contract;
use crate::contract_actions::admin;
use crate::contract_actions::token;

pub fn set_rewards(env: &Env, reward_types: &Map<Symbol, u32>) {
    env.storage().set(DataKey::Rewards, reward_types);
}

pub fn reward_member(env: &Env, approval_sign: &Signature, to: &AccountId, reward_type: &Symbol) {
    if !validation::is_member(&env, &to) {
        panic!("The user account you're trying to reward doesn't belong to the organization");
    }

    if !is_reward_valid(&env, &reward_type) {
        panic!("The reward type you are trying to use isn't supported")
    }

    let reward_value = get_reward_by_type(&env, &reward_type);
    transfer(&env, &approval_sign, &identifier_wrapper::get_account_identifier(to.clone()), &reward_value);
}

fn get_reward_by_type(env: &Env, r_type: &Symbol) -> i128 {
    let key = DataKey::Rewards;
    let rewards: Map<Symbol, i128> = env.storage().get(key).unwrap().unwrap();

    rewards.get(r_type.clone()).unwrap().unwrap()
}

fn transfer(env: &Env, approval_sign: &Signature, to: &Identifier, amount: &i128) {
    let tc_id = token_contract::get_token_contract_id(&env);
    let client = token::Client::new(&env, tc_id);

    let admin_id = admin::get_admin_id(&env);
    let nonce = client.nonce(&admin_id);

    client.xfer(&approval_sign, &nonce, &to, &amount);
}

fn get_rewards(env: &Env) -> Map<Symbol, u32> {
    let key = DataKey::Rewards;
    env.storage().get(key).unwrap().unwrap()
}

fn is_reward_valid(env: &Env, key: &Symbol) -> bool {
    let rewards = get_rewards(&env);
    rewards.contains_key(key.clone())
}
