#![no_std]

use soroban_sdk::{contractimpl, Env, Vec, BytesN, AccountId};

use soroban_auth::{Identifier};

mod contract_actions;

pub struct OrganizationContract;

pub trait OrganizationContractTrait {
    fn initialize(
        e: Env,
        admin: Identifier,
        token_c_id:BytesN<32>
    );

    /// add member to the organization
    fn add_m(env: Env, account: AccountId);

    /// revoke to the organization
    fn revoke_m(env: Env, from: AccountId);

    /// get token contract to the organization
    fn get_tc_id(env: Env) -> BytesN<32>;
    
    /// get members to the organization
    fn get_m(env: Env) -> Vec<AccountId>; 
}

#[contractimpl]
impl OrganizationContractTrait for OrganizationContract {
    fn initialize(
        env: Env, 
        admin: Identifier,
        token_c_id: BytesN<32>
    ) {
        contract_actions::admin::set_admin_id(&env, &admin);

        contract_actions::token_contract::set_token_id(&env, &token_c_id);
    }

    fn add_m(env: Env, account: AccountId) {
        contract_actions::member::add_member(&env, account);
    }
    
    fn revoke_m(env: Env, from: AccountId) {
        contract_actions::member::revoke_membership(&env, &from);
    }
    
    fn get_tc_id(env: Env) -> BytesN<32> {
        contract_actions::token_contract::get_token_contract_id(&env)
    }

    fn get_m(env: Env) -> Vec<AccountId> {
        contract_actions::member::get_members(&env)
    }
}
