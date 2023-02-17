//! Module for Organization
//! 
//! Module to set and get organizaion name
use soroban_sdk::{Env, Symbol};
use crate::contract_actions::datakey::DataKey;

pub fn get_organization_name(env: &Env) -> Symbol {
    env.storage().get(DataKey::OrgName).unwrap().unwrap()
}
