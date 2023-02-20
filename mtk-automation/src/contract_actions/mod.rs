//! Call each of the modules
pub mod admin;
pub mod datakey;
pub mod fund;
pub mod member;
pub mod organization;
pub mod reward;
pub mod token_contract;
pub mod token_operation;
pub mod token {
    soroban_sdk::contractimport!(file = "./token_contract.wasm");
}
