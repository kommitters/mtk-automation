//! Module for Organization
//!
//! Module to set and get organization name
use crate::contract_actions::datakey::DataKey;
use soroban_sdk::{Env, Symbol};

pub(crate) fn set_organization_name(env: &Env, new_value: &Symbol) {
    env.storage().set(&DataKey::OrgName, new_value);
}

pub(crate) fn get_organization_name(env: &Env) -> Symbol {
    env.storage().get(&DataKey::OrgName).unwrap().unwrap()
}
