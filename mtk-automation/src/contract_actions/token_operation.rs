//! Module Token Operation
//!
//! Module where the token_contract functions are used

use crate::contract_actions::admin;
use crate::contract_actions::token;
use crate::contract_actions::token_contract;
use soroban_sdk::{Address, Env};

use super::fund::get_available_funds_to_issue;

pub fn transfer(env: &Env, token_admin: &Address, to: &Address, amount: &i128) {
    let tc_id = token_contract::get_token_contract_id(env);
    let client = token::Client::new(env, &tc_id);

    client.xfer(token_admin, to, amount);
}

pub fn bring_back_tokens_to_admin(env: &Env, from: &Address) {
    let tc_id = token_contract::get_token_contract_id(env);
    let client = token::Client::new(env, &tc_id);

    let admin_id = admin::get_admin_id(env);
    let from_address = from.clone();
    let member_balance = client.balance(&from_address);

    swap_token_to();

    client.xfer_from(&admin_id, &from_address, &admin_id, &member_balance);
}

pub fn fund_contract_balance(env: &Env, token_admin: &Address) {
    let token_id = token_contract::get_token_contract_id(env);
    let admin_id = admin::get_admin_id(env);
    let token_client = token::Client::new(env, &token_id);

    token_client.mint(token_admin, &admin_id, &get_available_funds_to_issue(env));
}

fn swap_token_to() {
    todo!("Do the swapping process from MKT to other available token");
}
