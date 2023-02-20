#![cfg(test)]
extern crate std;

use super::{OrganizationContract, OrganizationContractClient};
use soroban_sdk::{symbol, testutils::Address as _, Address, BytesN, Env, Map, Symbol};

mod token {
    soroban_sdk::contractimport!(file = "./token_contract.wasm");
}

fn create_and_init_token_contract(env: &Env, admin_id: &Address) -> (BytesN<32>, token::Client) {
    let id = env.register_stellar_asset_contract(admin_id.clone());
    let token = token::Client::new(env, &id);
    (id, token)
}

#[test]
fn happy_path() {
    let env = Env::default();

    // USERS
    let token_admin = Address::random(&env);
    // John Doe
    let doe_user = Address::random(&env);

    // CREATE OUR CUSTOM CONTRACT
    let contract_id = env.register_contract(None, OrganizationContract);
    let contract_client = OrganizationContractClient::new(&env, &contract_id);

    // CREATE TOKEN CONTRACT
    let (token_id, token_client) = create_and_init_token_contract(&env, &token_admin);

    // Initializate Contract with initial values.
    let allowed_funds_to_issue = 10000;
    let org_name = symbol!("Kommit");
    let items = [(symbol!("thank"), 35), (symbol!("congrat"), 25)];
    let rewards: Map<Symbol, i32> = Map::from_array(&env, items);

    contract_client.initialize(
        &token_admin,
        &org_name,
        &rewards,
        &allowed_funds_to_issue,
        &token_id,
    );

    assert_eq!(
        contract_client.org_name(),
        org_name,
        "Correct name set on contract"
    );

    contract_client.fund_c(&token_admin);
    assert_eq!(
        contract_client.get_bal(),
        allowed_funds_to_issue,
        "Correct Funds found on contract"
    );

    contract_client.add_m(&doe_user);

    assert!(
        contract_client.get_m().contains(&doe_user),
        "Member was successfully added"
    );

    contract_client.reward_m(&token_admin, &doe_user, &symbol!("congrat"));
    contract_client.reward_m(&token_admin, &doe_user, &symbol!("thank"));
    assert_eq!(
        token_client.balance(&doe_user),
        60,
        "Contract admin gets back member funds"
    );
}

#[test]
#[should_panic(
    expected = "The user account you're trying to compensate doesn't belong to the organization"
)]
fn remove_no_member_account() {
    let env = Env::default();

    let token_admin = Address::random(&env);

    let doe_user = Address::random(&env);

    let contract_id = env.register_contract(None, OrganizationContract);
    let contract_client = OrganizationContractClient::new(&env, &contract_id);

    let (token_id, _token_client) = create_and_init_token_contract(&env, &token_admin);

    let allowed_funds_to_issue = 1000;
    let org_name = symbol!("Kommit");
    let items = [(symbol!("talk"), 35), (symbol!("blog_post"), 25)];
    let rewards: Map<Symbol, i32> = Map::from_array(&env, items);

    contract_client.initialize(
        &token_admin,
        &org_name,
        &rewards,
        &allowed_funds_to_issue,
        &token_id,
    );

    contract_client.fund_c(&token_admin);

    contract_client.reward_m(&token_admin, &doe_user, &symbol!("blog_post"));
}

#[test]
#[should_panic(expected = "The compensation type you are trying to use isn't supported")]
fn reward_with_invalid_type() {
    let env = Env::default();

    // USERS
    let token_admin = Address::random(&env);
    // John Doe
    let doe_user = Address::random(&env);

    let contract_id = env.register_contract(None, OrganizationContract);
    let contract_client = OrganizationContractClient::new(&env, &contract_id);

    let (token_id, _token_client) = create_and_init_token_contract(&env, &token_admin);

    let allowed_funds_to_issue = 1000;
    let org_name = symbol!("Kommit");
    let items = [(symbol!("talk"), 35), (symbol!("blog_post"), 25)];
    let rewards: Map<Symbol, i32> = Map::from_array(&env, items);

    contract_client.initialize(
        &token_admin,
        &org_name,
        &rewards,
        &allowed_funds_to_issue,
        &token_id,
    );

    contract_client.fund_c(&token_admin);
    contract_client.add_m(&doe_user);

    contract_client.reward_m(&token_admin, &doe_user, &symbol!("oss_contri"));
}