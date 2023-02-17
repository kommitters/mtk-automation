//! Module for Organization
//! 
//! Module for managment the fund of the organization
use soroban_sdk::Env;
use crate::contract_actions::datakey::DataKey;
use crate::contract_actions::admin;
use crate::contract_actions::token;
use crate::contract_actions::token_contract;

pub fn set_available_funds_to_issue(env: &Env, new_value: i128){
    env.storage().set(DataKey::AllowedF, new_value);
}

pub fn get_available_funds_to_issue(env: &Env) -> i128 {
    env.storage().get(DataKey::AllowedF).unwrap().unwrap()
}

pub fn get_contract_balance(env: &Env) -> i128 {
    let tc_id = token_contract::get_token_contract_id(&env);
    let client = token::Client::new(&env, tc_id);

    let admin_id = admin::get_admin_id(&env);

    client.balance(&admin_id)
}
