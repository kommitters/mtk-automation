#![no_std]

mod contract_actions;

use crate::contract_actions::{
    admin, fund, member, offset, organization, token_contract, token_operation,
};
use soroban_sdk::{contractimpl, Address, BytesN, Env, Map, Symbol, Vec};

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
    fn add_m(env: Env, account: Address, admin: Address);

    /// revoke to the organization
    fn revoke_m(env: Env, from: Address, admin: Address);

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
        if admin::has_administrator(&env) {panic!("Contract already initialized")}
        admin::set_admin_id(&env, &admin);
        organization::set_organization_name(&env, &org_name);
        fund::set_available_funds_to_issue(&env, &fund_amount);
        token_contract::set_token_id(&env, &token_c_id);
        offset::set_offset(&env, &offsets);
    }

    fn add_m(env: Env, account: Address, admin: Address) {
        member::add_member(&env, account, admin);
    }

    fn revoke_m(env: Env, from: Address, admin: Address) {
        member::revoke_membership(&env, &from, admin);
    }

    fn offset_m(env: Env, admin_address: Address, to: Address, o_type: Symbol) {
        offset::offset_a_member(&env, &admin_address, &to, &o_type);
    }

    fn get_tc_id(env: Env) -> BytesN<32> {
        token_contract::get_token_contract_id(&env)
    }

    fn get_bal(env: Env) -> i128 {
        fund::get_contract_balance(&env)
    }

    fn org_name(env: Env) -> Symbol {
        organization::get_organization_name(&env)
    }

    fn fund_c(env: Env, admin_address: Address) {
        token_operation::fund_contract_balance(&env, &admin_address);
    }

    fn get_m(env: Env) -> Vec<Address> {
        member::get_members(&env)
    }
}

mod test;
