//! Validation Module
//! 
//! Module for contract validation management
use soroban_sdk::{Env, AccountId, Vec};
use crate::contract_actions::member;

pub fn is_member(env: &Env, to: &AccountId) -> bool {
    let members: Vec<AccountId> = member::get_members(&env);
    members.contains(to)
} 
