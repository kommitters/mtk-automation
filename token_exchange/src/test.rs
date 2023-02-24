#![cfg(test)]
extern crate std;

use crate::{actions::token, TokenTradeClient};
use soroban_sdk::{symbol, testutils::Address as _, Address, BytesN, Env, IntoVal};

fn create_and_init_token_contract(env: &Env, admin_id: &Address) -> (BytesN<32>, token::Client) {
    let id = env.register_stellar_asset_contract(admin_id.clone());
    let token = token::Client::new(env, &id);
    (id, token)
}

fn create_single_offer_contract(
    env: &Env,
    seller: &Address,
    sell_token: &BytesN<32>,
    buy_token: &BytesN<32>,
    sell_price: u32,
    buy_price: u32,
) -> TokenTradeClient {
    let offer = TokenTradeClient::new(env, &env.register_contract(None, crate::TokenTrade {}));
    offer.create(seller, sell_token, buy_token, &sell_price, &buy_price);

    // Verify that authorization is required for the seller.
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            seller.clone(),
            offer.contract_id.clone(),
            symbol!("create"),
            (
                seller,
                sell_token.clone(),
                buy_token.clone(),
                sell_price,
                buy_price
            )
                .into_val(env)
        )]
    );

    offer
}

#[test]
fn succesfull_trade() {
    let env: Env = Default::default();
    let token_admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let (sell_token_id, sell_token_client) = create_and_init_token_contract(&env, &token_admin);
    let (buy_token_id, buy_token_client) = create_and_init_token_contract(&env, &token_admin);

    // The price here is 1 sell_token for 1 buy_token.
    let offer = create_single_offer_contract(&env, &seller, &sell_token_id, &buy_token_id, 1, 1);
    let offer_address = Address::from_contract_id(&env, &offer.contract_id);
    sell_token_client.mint(&token_admin, &seller, &200);
    buy_token_client.mint(&token_admin, &buyer, &100);

    //Contract owner need to "mint" the contract if we want the contract to be the middleman
    offer.mint_cont(&sell_token_id, &100);
    assert_eq!(offer.get_c_bal(&sell_token_id), 100);

    offer.trade_btc(&buyer, &10_i128);
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            buyer.clone(),
            offer.contract_id.clone(),
            symbol!("trade_btc"),
            (&buyer, 10_i128).into_val(&env)
        )]
    );
    offer.trade_ctb(&buyer);
    offer.trade_cts();

    assert_eq!(sell_token_client.balance(&buyer), 10);
    assert_eq!(buy_token_client.balance(&seller), 10);
    assert_eq!(sell_token_client.balance(&offer_address), 90);
    assert_eq!(buy_token_client.balance(&buyer), 90);
    assert_eq!(buy_token_client.balance(&offer_address), 0);

    offer.withdraw(&sell_token_id, &20);
    assert_eq!(sell_token_client.balance(&offer_address), 70);

    // The price here is 1 sell_token for 2 buy_token.
    offer.updt_price(&seller, &1, &2);

    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            seller.clone(),
            offer.contract_id.clone(),
            symbol!("updt_price"),
            (&seller, 1_u32, 2_u32).into_val(&env)
        )]
    );

    offer.trade_btc(&buyer, &20);
    offer.trade_ctb(&buyer);
    offer.trade_cts();
    assert_eq!(sell_token_client.balance(&buyer), 20);
    assert_eq!(buy_token_client.balance(&seller), 30);
    assert_eq!(sell_token_client.balance(&offer_address), 60);

    assert_eq!(offer.get_offer().seller, seller);
}

#[test]
#[should_panic(expected = "offer is already created")]
fn create_when_contract_already_created() {
    let env: Env = Default::default();
    let token_admin = Address::random(&env);
    let seller = Address::random(&env);
    let (sell_token_id, _) = create_and_init_token_contract(&env, &token_admin);
    let (buy_token_id, _) = create_and_init_token_contract(&env, &token_admin);

    let offer = TokenTradeClient::new(&env, &env.register_contract(None, crate::TokenTrade {}));
    offer.create(&seller, &sell_token_id, &buy_token_id, &1, &10);
    offer.create(&seller, &sell_token_id, &buy_token_id, &1, &1);
}

#[test]
#[should_panic(expected = "zero price is not allowed")]
fn create_with_not_allowed_prices() {
    let env: Env = Default::default();
    let token_admin = Address::random(&env);
    let seller = Address::random(&env);
    let (sell_token_id, _) = create_and_init_token_contract(&env, &token_admin);
    let (buy_token_id, _) = create_and_init_token_contract(&env, &token_admin);

    create_single_offer_contract(&env, &seller, &sell_token_id, &buy_token_id, 0, 0);
}

#[test]
#[should_panic(expected = "zero price is not allowed")]
fn update_with_not_allowed_prices() {
    let env: Env = Default::default();
    let token_admin = Address::random(&env);
    let seller = Address::random(&env);
    let (sell_token_id, _) = create_and_init_token_contract(&env, &token_admin);
    let (buy_token_id, _) = create_and_init_token_contract(&env, &token_admin);

    let offer = create_single_offer_contract(&env, &seller, &sell_token_id, &buy_token_id, 1, 1);
    offer.updt_price(&seller, &0, &0);
}

#[test]
#[should_panic(expected = "The contract administrator is not the address specified")]
fn update_without_being_admin() {
    let env: Env = Default::default();
    let token_admin = Address::random(&env);
    let seller = Address::random(&env);
    let buyer = Address::random(&env);
    let (sell_token_id, _) = create_and_init_token_contract(&env, &token_admin);
    let (buy_token_id, _) = create_and_init_token_contract(&env, &token_admin);

    let offer = create_single_offer_contract(&env, &seller, &sell_token_id, &buy_token_id, 1, 1);
    offer.updt_price(&buyer, &1, &1);
}
