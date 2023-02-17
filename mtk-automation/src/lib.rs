#![no_std]

use soroban_sdk::{contractimpl, Env, Symbol, Vec, BytesN, AccountId, Map};

use soroban_auth::{Identifier, Signature};

mod contract_actions;

pub struct OrganizationContract;

pub trait OrganizationContractTrait {
    fn initialize(
        e: Env,
        admin: Identifier,
        org_name: Symbol,
        rewards: Map<Symbol, u32>,
        fund_amount: i128,
        token_c_id:BytesN<32>
    );

    /// add member to the organization
    fn add_m(env: Env, account: AccountId);

    /// revoke to the organization
    fn revoke_m(env: Env, from: AccountId);

    /// reward a member of the organization
    fn reward_m(e: Env, token_approval_sig: Signature, to: AccountId, r_type: Symbol);

    /// get token contract to the organization
    fn get_tc_id(env: Env) -> BytesN<32>;

    /// get the organization balance
    fn get_bal(env: Env) -> i128;
    
    /// get members to the organization
    fn get_m(env: Env) -> Vec<AccountId>;

    /// get the organization's name
    fn org_name(env: Env) -> Symbol;
    
    /// fund contract balance for the organization
    fn fund_c(env: Env, approval_sign: Signature);   
}

#[contractimpl]
impl OrganizationContractTrait for OrganizationContract {
    fn initialize(
        env: Env, 
        admin: Identifier,
        org_name: Symbol,
        rewards: Map<Symbol, u32>,
        fund_amount: i128,
        token_c_id: BytesN<32>
    ) {
        contract_actions::admin::set_admin_id(&env, &admin);

        contract_actions::organization::set_organization_name(&env, org_name);

        contract_actions::fund::set_available_funds_to_issue(&env, fund_amount);

        contract_actions::token_contract::set_token_id(&env, &token_c_id);

        contract_actions::reward::set_rewards(&env, &rewards);
    }

    fn add_m(env: Env, account: AccountId) {
        contract_actions::member::add_member(&env, account);
    }
    
    fn revoke_m(env: Env, from: AccountId) {
        contract_actions::member::revoke_membership(&env, &from);
    }

    fn reward_m(env: Env, approval_sign: Signature, to: AccountId, r_type: Symbol) {
        contract_actions::reward::reward_member(&env, &approval_sign, &to, &r_type);
    }
    
    fn get_tc_id(env: Env) -> BytesN<32> {
        contract_actions::token_contract::get_token_contract_id(&env)
    }

    fn get_bal(env: Env) -> i128 {
        contract_actions::fund::get_contract_balance(&env)
    }

    fn org_name(env: Env) -> Symbol {
        contract_actions::organization::get_organization_name(&env)
    }

    fn fund_c(env: Env, approval_sign: Signature) {
        contract_actions::token_operation::fund_contract_balance(&env, &approval_sign);
    }

    fn get_m(env: Env) -> Vec<AccountId> {
        contract_actions::member::get_members(&env)
    }
}
