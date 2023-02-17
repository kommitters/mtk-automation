//! Module for Organization
//! 
//! Module to set and get organization name
use soroban_sdk::{Env, Symbol};
use crate::contract_actions::datakey::DataKey;

pub fn set_organization_name(env: &Env, new_value: Symbol) {
    env.storage().set(DataKey::OrgName, new_value);
}

pub fn get_organization_name(env: &Env) -> Symbol {
    env.storage().get(DataKey::OrgName).unwrap().unwrap()
}
