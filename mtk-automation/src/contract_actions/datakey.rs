//! Module DataKey
//!
//! Module for defining the types of keys within the contract
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    OrgName,
    TokenId,
    AdminId,
    Offsets,
    Members,
    AllowedF, //Funds allowed for distribution
    ExId,     //Exchange contract identification
}
