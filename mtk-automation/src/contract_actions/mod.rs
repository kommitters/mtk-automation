//! Call each of the modules
pub mod datakey;
pub mod member;
pub mod token_contract;
pub mod admin;
pub mod identifier_wrapper;
pub mod reward;
pub mod token_operation;
pub mod fund;
pub mod organization;
pub mod token {
  soroban_sdk::contractimport!(file = "./soroban_token_spec.wasm");
}
