#![cfg(test)]
extern crate std;

use crate::contract_actions::token;

use super::{OrganizationContract, OrganizationContractClient};
mod token_exchange {
    soroban_sdk::contractimport!(file = "./token_exchange.wasm");
use soroban_sdk::{symbol, testutils::Address as _, Address, BytesN, Env, IntoVal, Map, Symbol};

mod token {
    soroban_sdk::contractimport!(file = "./token_contract.wasm");
}
const WASM: &[u8] = include_bytes!("../token_exchange.wasm");
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
) -> token_exchange::Client {
    let offer = token_exchange::Client::new(env, &env.register_contract_wasm(None, WASM));
    offer.create(seller, sell_token, buy_token, &sell_price, &buy_price);
    offer
}

#[test]
fn successfully_add_and_offset_a_member() {
    let env = Env::default();

    // USERS
    let admin_address = Address::random(&env);
    // John Doe
    let doe_user = Address::random(&env);

    // CREATE OUR CUSTOM CONTRACT
    let contract_id = env.register_contract(None, OrganizationContract);
    let contract_client = OrganizationContractClient::new(&env, &contract_id);

    // CREATE TOKEN CONTRACT
    let (token_id, token_client) = create_and_init_token_contract(&env, &admin_address);
    let (stable_token_id, stable_token_client) =
        create_and_init_token_contract(&env, &admin_address);
    stable_token_client.mint(&admin_address, &admin_address, &2000);

    // CREATE EXCHANGE CONTRACT
    let offer =
        create_single_offer_contract(&env, &admin_address, &stable_token_id, &token_id, 1, 1);
    offer.mint_cont(&stable_token_id, &1000);

    // Initializate Contract with initial values.
    let allowed_funds_to_issue = 10000;
    let org_name = symbol!("Kommit");
    let items = [(symbol!("thank"), 35), (symbol!("congrat"), 25)];
    let offsets: Map<Symbol, i32> = Map::from_array(&env, items);

    contract_client.initialize(
        &admin_address,
        &org_name,
        &offsets,
        &allowed_funds_to_issue,
        &token_id,
        &offer.contract_id,
    );

    assert_eq!(
        contract_client.org_name(),
        org_name,
        "Correct name set on contract"
    );

    contract_client.fund_c(&admin_address);

    assert_eq!(
        contract_client.get_bal(),
        allowed_funds_to_issue,
        "Correct Funds found on contract"
    );

    assert_eq!(
        contract_client.get_tc_id(),
        token_id,
        "Correct token id found on contract"
    );

    contract_client.add_m(&doe_user, &admin_address);
    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            admin_address.clone(),
            contract_id.clone(),
            symbol!("add_m"),
            (&doe_user, &admin_address).into_val(&env)
        )]
    );

    assert!(
        contract_client.get_m().contains(&doe_user),
        "Member was successfully added"
    );

    contract_client.offset_m(&admin_address, &doe_user, &symbol!("congrat"));

    assert_eq!(
        env.recorded_top_authorizations(),
        std::vec![(
            admin_address.clone(),
            contract_id,
            symbol!("offset_m"),
            (&admin_address, &doe_user, symbol!("congrat")).into_val(&env)
        )]
    );
    contract_client.offset_m(&admin_address, &doe_user, &symbol!("thank"));
    assert_eq!(
        token_client.balance(&doe_user),
        60,
        "Contract admin gets back member funds"
    );

    contract_client.revoke_m1(&doe_user);
    contract_client.revoke_m2(&doe_user);
    contract_client.revoke_m3(&doe_user);
    assert!(
        contract_client.get_m().is_empty(),
        "Member successfully revoked",
    );
    assert_eq!(token_client.balance(&doe_user), 0);
    assert_eq!(stable_token_client.balance(&doe_user), 60);
}

#[test]
#[should_panic(
    expected = "The user account you're trying to offset doesn't belong to the organization"
)]
fn remove_no_member_account() {
    let env = Env::default();

    let admin_address = Address::random(&env);

    let doe_user = Address::random(&env);

    let contract_id = env.register_contract(None, OrganizationContract);
    let contract_client = OrganizationContractClient::new(&env, &contract_id);

    let (token_id, _token_client) = create_and_init_token_contract(&env, &admin_address);
    let (stable_token_id, _stable_token_client) =
        create_and_init_token_contract(&env, &admin_address);

    let offer =
        create_single_offer_contract(&env, &admin_address, &token_id, &stable_token_id, 1, 1);

    let allowed_funds_to_issue = 1000;
    let org_name = symbol!("Kommit");
    let items = [(symbol!("talk"), 35), (symbol!("blog_post"), 25)];
    let offsets: Map<Symbol, i32> = Map::from_array(&env, items);

    contract_client.initialize(
        &admin_address,
        &org_name,
        &offsets,
        &allowed_funds_to_issue,
        &token_id,
        &offer.contract_id,
    );

    contract_client.fund_c(&admin_address);

    contract_client.offset_m(&admin_address, &doe_user, &symbol!("blog_post"));
}

#[test]
#[should_panic(expected = "The offset type you are trying to use isn't supported")]
fn offset_with_invalid_type() {
    let env = Env::default();

    // USERS
    let admin_address = Address::random(&env);
    // John Doe
    let doe_user = Address::random(&env);

    let contract_id = env.register_contract(None, OrganizationContract);
    let contract_client = OrganizationContractClient::new(&env, &contract_id);

    let (token_id, _token_client) = create_and_init_token_contract(&env, &admin_address);
    let (stable_token_id, _stable_token_client) =
        create_and_init_token_contract(&env, &admin_address);

    // CREATE EXCHANGE CONTRACT
    let offer =
        create_single_offer_contract(&env, &admin_address, &token_id, &stable_token_id, 1, 1);

    let allowed_funds_to_issue = 1000;
    let org_name = symbol!("Kommit");
    let items = [(symbol!("talk"), 35), (symbol!("blog_post"), 25)];
    let offsets: Map<Symbol, i32> = Map::from_array(&env, items);

    contract_client.initialize(
        &admin_address,
        &org_name,
        &offsets,
        &allowed_funds_to_issue,
        &token_id,
        &offer.contract_id,
    );

    contract_client.fund_c(&admin_address);
    contract_client.add_m(&doe_user, &admin_address);

    contract_client.offset_m(&admin_address, &doe_user, &symbol!("oss_contri"));
}
