#![no_std]

use soroban_sdk::{contractimpl, Address, BytesN, Env, Map, Symbol, Vec};

mod contract_actions;

pub struct OrganizationContract;

pub trait OrganizationContractTrait {
    fn initialize(
        e: Env,
        admin: Address,
        org_name: Symbol,
        rewards: Map<Symbol, u32>,
        fund_amount: i128,
        token_c_id: BytesN<32>,
    );

    /// add member to the organization
    fn add_m(env: Env, account: Address);

    /// revoke to the organization
    fn revoke_m(env: Env, from: Address);

    /// reward a member of the organization
    fn reward_m(e: Env, token_address: Address, to: Address, r_type: Symbol);

    /// get token contract to the organization
    fn get_tc_id(env: Env) -> BytesN<32>;

    /// get the organization balance
    fn get_bal(env: Env) -> i128;

    /// get members to the organization
    fn get_m(env: Env) -> Vec<Address>;

    /// get the organization's name
    fn org_name(env: Env) -> Symbol;

    /// fund contract balance for the organization
    fn fund_c(env: Env, approval_sign: Address);
}

#[contractimpl]
impl OrganizationContractTrait for OrganizationContract {
    fn initialize(
        env: Env,
        admin: Address,
        org_name: Symbol,
        rewards: Map<Symbol, u32>,
        fund_amount: i128,
        token_c_id: BytesN<32>,
    ) {
        contract_actions::admin::set_admin_id(&env, &admin);

        contract_actions::organization::set_organization_name(&env, org_name);

        contract_actions::fund::set_available_funds_to_issue(&env, fund_amount);

        contract_actions::token_contract::set_token_id(&env, &token_c_id);

        contract_actions::reward::set_rewards(&env, &rewards);
    }

    fn add_m(env: Env, account: Address) {
        contract_actions::member::add_member(&env, account);
    }

    fn revoke_m(env: Env, from: Address) {
        contract_actions::member::revoke_membership(&env, &from);
    }

    fn reward_m(env: Env, approval_address: Address, to: Address, c_type: Symbol) {
        contract_actions::reward::reward_member(&env, &approval_address, &to, &c_type);
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

    fn fund_c(env: Env, approval_address: Address) {
        contract_actions::token_operation::fund_contract_balance(&env, &approval_address);
    }

    fn get_m(env: Env) -> Vec<Address> {
        contract_actions::member::get_members(&env)
    }
}
