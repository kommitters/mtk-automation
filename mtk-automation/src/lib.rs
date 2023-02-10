#![no_std]

use soroban_sdk::{contractimpl, Env, Vec, BytesN, AccountId};

use soroban_auth::Identifier;

mod modules;

pub struct OrganizationContract;

pub trait OrganizationContractTrait {
    fn initialize(
        e: Env,
        admin: Identifier,
        token_c_id:BytesN<32>
    );

    fn add_m(env: Env, account: AccountId);

    fn revoke_m(env: Env, from: AccountId);

    fn get_tc_id(env: Env) -> BytesN<32>;
    
    fn get_m(env: Env) -> Vec<AccountId>;
}

#[contractimpl]
impl OrganizationContractTrait for OrganizationContract {
    fn initialize(
        env: Env, 
        admin: Identifier,
        token_c_id: BytesN<32>
    ) {
        modules::admin::admin::set_admin_id(&env, &admin);

        modules::token_contract::token_contract::set_token_id(&env, &token_c_id);
    }

    fn add_m(env: Env, account: AccountId) {
        modules::member::member::add_member(&env, account);
    }
    
    fn revoke_m(env: Env, from: AccountId) {
        modules::member::member::revoke_membership(&env, &from);
    }
    
    fn get_tc_id(env: Env) -> BytesN<32> {
        modules::token_contract::token_contract::get_token_contract_id(&env)
    }

    fn get_m(env: Env) -> Vec<AccountId> {
        modules::member::member::get_members(&env)
    }
}
