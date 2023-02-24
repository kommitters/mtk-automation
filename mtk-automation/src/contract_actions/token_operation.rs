//! Module Token Operation
//!
//! Module where the token_contract functions are used
mod token_exchange {
    soroban_sdk::contractimport!(file = "./token_exchange.wasm");
}

use crate::contract_actions::{
    admin, exchange_contract::get_exchange_contract_id, token, token_contract,
};
use soroban_sdk::{Address, BytesN, Env};

use super::fund::get_available_funds_to_issue;

pub(crate) fn transfer(env: &Env, admin_address: &Address, to: &Address, amount: &i128) {
    admin_address.require_auth();
    let tc_id = token_contract::get_token_contract_id(env);
    let client = token::Client::new(env, &tc_id);
    client.xfer(admin_address, to, amount);
}

pub(crate) fn bring_back_tokens_to_admin(env: &Env, from: &Address) {
    from.require_auth();
    let tc_id = token_contract::get_token_contract_id(env);
    let client = token::Client::new(env, &tc_id);

    let from_address = from.clone();
    let member_balance = client.balance(&from_address);
    let exchange_contract_id: BytesN<32> = get_exchange_contract_id(env);
    let exchange_client = token_exchange::Client::new(env, &exchange_contract_id);
    exchange_client.trade_btc(&from_address, &member_balance);
}

pub(crate) fn bring_back_tokens_to_admin2(env: &Env, from: &Address) {
    let from_address = from.clone();
    let exchange_contract_id: BytesN<32> = get_exchange_contract_id(env);
    let exchange_client = token_exchange::Client::new(env, &exchange_contract_id);

    exchange_client.trade_ctb(&from_address);
}

pub(crate) fn bring_back_tokens_to_admin3(env: &Env) {
    let exchange_contract_id: BytesN<32> = get_exchange_contract_id(env);
    let exchange_client = token_exchange::Client::new(env, &exchange_contract_id);
    exchange_client.trade_cts();
}

pub(crate) fn fund_contract_balance(env: &Env, admin_address: &Address) {
    admin_address.require_auth();
    let token_id = token_contract::get_token_contract_id(env);
    let admin_id = admin::get_admin_id(env);
    let token_client = token::Client::new(env, &token_id);

    token_client.mint(admin_address, &admin_id, &get_available_funds_to_issue(env));
}
