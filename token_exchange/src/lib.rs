//! This contract implements trading of one token pair between one seller and
//! multiple buyers
#![no_std]

mod actions;
use actions::{contract_token, offer::Offer, storage, storage_types::DataKey, trade};
use soroban_sdk::{contractimpl, Address, BytesN, Env};

pub trait TokenTradeTrait {
    fn create(
        e: Env,
        seller: Address,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
        sell_price: u32,
        buy_price: u32,
    );
    fn trade_btc(e: Env, buyer: Address, buy_token_amount: i128);
    fn trade_ctb(env: Env, buyer: Address);
    fn trade_cts(env: Env, buyer: Address);
    fn mint_cont(env: Env, token: BytesN<32>, amount: i128);
    fn withdraw(e: Env, token: BytesN<32>, amount: i128);
    fn updt_price(e: Env, sell_price: u32, buy_price: u32);
    fn get_offer(e: Env) -> Offer;
    fn get_c_bal(env: Env, token: BytesN<32>) -> i128;
}

pub struct TokenTrade;

#[contractimpl]
impl TokenTradeTrait for TokenTrade {
    /// Creates the offer for seller for the given token pair and initial price.
    fn create(
        env: Env,
        seller: Address,
        sell_token: BytesN<32>,
        buy_token: BytesN<32>,
        sell_price: u32,
        buy_price: u32,
    ) {
        if env.storage().has(&DataKey::Offer) {
            panic!("offer is already created");
        }
        if buy_price == 0 || sell_price == 0 {
            panic!("zero price is not allowed");
        }

        seller.require_auth();
        storage::write_offer(
            &env,
            &Offer {
                seller,
                sell_token,
                buy_token,
                sell_price,
                buy_price,
            },
        );
    }

    /// Trades `buy_token_amount` of buy_token from buyer for `sell_token` amount
    /// defined by the price.
    /// Buyer needs to authorize the `trade` call and internal `xfer` call to
    /// the contract address.
    /// Divided in 3 function for demonstration purposes
    /// until the cli allows multiple signs in the same function
    fn trade_btc(env: Env, buyer: Address, buy_token_amount: i128) {
        trade::trade_buyer_to_contract(&env, &buyer, &buy_token_amount)
    }

    fn trade_ctb(env: Env, buyer: Address) {
        trade::trade_contract_to_buyer(&env, &buyer)
    }

    fn trade_cts(env: Env, buyer: Address) {
        trade::trade_contract_to_seller(&env, &buyer)
    }

    /// Sends amount of token from this contract to the seller.
    /// This is intentionally flexible so that the seller can withdraw any
    /// outstanding balance of the contract.
    /// Must be authorized by seller.
    fn withdraw(env: Env, token: BytesN<32>, amount: i128) {
        contract_token::withdraw_contract_balance(&env, &token, &amount)
    }

    ///Allows to send an amount of any token to the contract
    ///so it can make trades with it
    fn mint_cont(env: Env, token: BytesN<32>, amount: i128) {
        contract_token::mint_contract_balance(&env, &token, &amount)
    }

    ///Allows to request contract balance for the specified token
    fn get_c_bal(env: Env, token: BytesN<32>) -> i128 {
        contract_token::get_contract_balance(&env, &token)
    }

    /// Updates the price.
    /// Must be authorized by seller.
    fn updt_price(env: Env, sell_price: u32, buy_price: u32) {
        storage::update_offer_price(&env, &sell_price, &buy_price)
    }

    /// Returns the current state of the offer.
    fn get_offer(e: Env) -> Offer {
        storage::load_offer(&e)
    }
}

mod test;
