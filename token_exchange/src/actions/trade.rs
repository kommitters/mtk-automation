//! Module Trade
//! 
//!A module that has the 3 trading steps at the time of making a trading process, 
//!only for demonstration purposes, when the CLI allows making multiple 
//!transactions in a single function it should be changed
use soroban_sdk::{Address, Env};

use super::{storage, token};

pub(crate) fn trade_buyer_to_contract(env: &Env, buyer: &Address, buy_token_amount: &i128) {
    buyer.require_auth();
    let offer = storage::load_offer(env);
    let buy_token_client = token::Client::new(env, &offer.buy_token);
    let contract = env.current_contract_address();
    storage::store_sell_token_amount(env, offer, buy_token_amount);
    storage::set_buy_token_amount(env, buy_token_amount);

    buy_token_client.xfer(buyer, &contract, buy_token_amount);
}

pub(crate) fn trade_contract_to_buyer(env: &Env, buyer: &Address) {
    let offer = storage::load_offer(env);
    let contract = env.current_contract_address();
    let sell_token_client = token::Client::new(env, &offer.sell_token);
    let sell_token_amount = storage::get_sell_token_amount(env);

    sell_token_client.xfer(&contract, buyer, &sell_token_amount);
}

pub(crate) fn trade_contract_to_seller(env: &Env) {
    let offer = storage::load_offer(env);
    let contract = env.current_contract_address();
    let buy_token_client = token::Client::new(env, &offer.buy_token);
    let buy_token_amount = storage::get_buy_token_amount(env);

    buy_token_client.xfer(&contract, &offer.seller, &buy_token_amount);
}
