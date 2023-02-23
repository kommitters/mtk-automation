//! Module Token Operation
//!
//! Module where the token_contract functions are used
mod token_exchange {
    soroban_sdk::contractimport!(file = "./token_exchange.wasm");
}
use crate::contract_actions::{admin, datakey::DataKey, token, token_contract};
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

    let admin_id = admin::get_admin_id(env);
    let from_address = from.clone();
    let member_balance = client.balance(&from_address);

    swap_token_to(env, &from_address, &member_balance);

    client.xfer_from(&admin_id, &from_address, &admin_id, &member_balance);
}

pub(crate) fn fund_contract_balance(env: &Env, admin_address: &Address) {
    admin_address.require_auth();
    let token_id = token_contract::get_token_contract_id(env);
    let admin_id = admin::get_admin_id(env);
    let token_client = token::Client::new(env, &token_id);

    token_client.mint(admin_address, &admin_id, &get_available_funds_to_issue(env));
}

fn swap_token_to(env: &Env, from_address: &Address, member_balance: &i128) {
    // todo!("Do the swapping process from MKT to other available token");
    let exchange_contract_id: BytesN<32> = env.storage().get(&DataKey::ExId).unwrap().unwrap();
    let client = token_exchange::Client::new(env, &exchange_contract_id);
    client.trade_btc(from_address, member_balance);
    client.trade_ctb(from_address);
    client.trade_cts();
}
