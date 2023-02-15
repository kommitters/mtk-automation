//! Call each of the modules
pub mod datakey;
pub mod member;
pub mod token_contract;
pub mod admin;
pub mod identifier_wrapper;
pub mod compensate;
pub mod token_operation;
pub mod token {
  soroban_sdk::contractimport!(file = "./soroban_token_spec.wasm");
}
