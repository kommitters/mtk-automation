#![no_std]

use soroban_sdk::{contractimpl, Address, BytesN, Env, Map, Symbol, Vec};

mod contract_actions;

pub struct OrganizationContract;

pub trait OrganizationContractTrait {
    fn initialize(
        e: Env,
        admin: Address,
        org_name: Symbol,
        offsets: Map<Symbol, i32>,
        fund_amount: i128,
        token_c_id: BytesN<32>,
    );

    /// add member to the organization
    fn add_m(env: Env, account: Address);

    /// revoke to the organization
    fn revoke_m(env: Env, from: Address);

    /// offset a member of the organization
    fn offset_m(e: Env, token_address: Address, to: Address, o_type: Symbol);

    /// get token contract to the organization
    fn get_tc_id(env: Env) -> BytesN<32>;

    /// get the organization balance
    fn get_bal(env: Env) -> i128;

    /// get members to the organization
    fn get_m(env: Env) -> Vec<Address>;

    /// get the organization's name
    fn org_name(env: Env) -> Symbol;

    /// fund contract balance for the organization
    fn fund_c(env: Env, admin_address: Address);
}

#[contractimpl]
impl OrganizationContractTrait for OrganizationContract {
    fn initialize(
        env: Env,
        admin: Address,
        org_name: Symbol,
        offsets: Map<Symbol, i32>,
        fund_amount: i128,
        token_c_id: BytesN<32>,
    ) {
        contract_actions::admin::set_admin_id(&env, &admin);

        contract_actions::organization::set_organization_name(&env, org_name);

        contract_actions::fund::set_available_funds_to_issue(&env, fund_amount);

        contract_actions::token_contract::set_token_id(&env, &token_c_id);

        contract_actions::offset::set_offset(&env, &offsets);
    }

    fn add_m(env: Env, account: Address) {
        contract_actions::member::add_member(&env, account);
    }

    fn revoke_m(env: Env, from: Address) {
        contract_actions::member::revoke_membership(&env, &from);
    }

    fn offset_m(env: Env, admin_address: Address, to: Address, o_type: Symbol) {
        contract_actions::offset::offset_a_member(&env, &admin_address, &to, &o_type);
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

    fn fund_c(env: Env, admin_address: Address) {
        contract_actions::token_operation::fund_contract_balance(&env, &admin_address);
    }

    fn get_m(env: Env) -> Vec<Address> {
        contract_actions::member::get_members(&env)
    }
}

mod test;
