//!Module Storage
//!
//!Allows to request and update the storage values of the contract

use soroban_sdk::{unwrap::UnwrapOptimized, Env};

use super::{offer::Offer, storage_types::DataKey};

pub(crate) fn load_offer(e: &Env) -> Offer {
    e.storage().get_unchecked(&DataKey::Offer).unwrap()
}

pub(crate) fn write_offer(e: &Env, offer: &Offer) {
    e.storage().set(&DataKey::Offer, offer)
}

pub(crate) fn get_sell_token_amount(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::SellTokenA).unwrap()
}

pub(crate) fn set_sell_token_amount(env: &Env, amount: &i128) {
    env.storage().set(&DataKey::SellTokenA, amount)
}

pub(crate) fn get_buy_token_amount(e: &Env) -> i128 {
    e.storage().get_unchecked(&DataKey::BuyTokenA).unwrap()
}

pub(crate) fn set_buy_token_amount(env: &Env, amount: &i128) {
    env.storage().set(&DataKey::BuyTokenA, amount)
}

// Compute the amount of token that buyer needs to receive
pub(crate) fn store_sell_token_amount(env: &Env, offer: Offer, buy_token_amount: &i128) {
    let sell_token_amount = buy_token_amount
        .checked_mul(offer.sell_price as i128)
        .unwrap_optimized()
        / offer.buy_price as i128;
    set_sell_token_amount(env, &sell_token_amount);
}

pub(crate) fn update_offer_price(env: &Env, sell_price: &u32, buy_price: &u32) {
    if *buy_price == 0 || *sell_price == 0 {
        panic!("zero price is not allowed");
    }
    let mut offer = load_offer(env);

    offer.seller.require_auth();
    offer.sell_price = *sell_price;
    offer.buy_price = *buy_price;
    write_offer(env, &offer);
}
