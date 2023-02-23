use soroban_sdk::{BytesN, Env};

use super::{storage, token};

pub(crate) fn withdraw_contract_balance(env: &Env, token: &BytesN<32>, amount: &i128) {
    let offer = storage::load_offer(env);
    offer.seller.require_auth();
    token::Client::new(env, token).xfer(&env.current_contract_address(), &offer.seller, amount);
}

pub(crate) fn mint_contract_balance(env: &Env, token: &BytesN<32>, amount: &i128) {
    let offer = storage::load_offer(env);
    offer.seller.require_auth();
    token::Client::new(env, token).xfer(&offer.seller, &env.current_contract_address(), amount);
}

pub(crate) fn get_contract_balance(env: &Env, token: &BytesN<32>) -> i128 {
    let offer = storage::load_offer(env);
    let contract = env.current_contract_address();
    offer.seller.require_auth();
    token::Client::new(env, token).balance(&contract)
}
