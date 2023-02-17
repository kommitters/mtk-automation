//! Module Token Operation
//! 
//! Module where the token_contract functions are used
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{Env, AccountId};
use crate::contract_actions::token_contract;
use crate::contract_actions::admin;
use crate::contract_actions::token;
use crate::contract_actions::fund;
use crate::contract_actions::identifier_wrapper as identifier;

pub fn transfer(env: &Env, approval_sign: &Signature, to: &Identifier, amount: &i128) {
  let tc_id = token_contract::get_token_contract_id(env);
  let client = token::Client::new(env, tc_id);

  let admin_id = admin::get_admin_id(env);
  let nonce = client.nonce(&admin_id);

  client.xfer(approval_sign, &nonce, to, amount);
}

pub fn bring_back_tokens_to_admin(env: &Env, from: &AccountId) {
  let tc_id = token_contract::get_token_contract_id(env);
  let client = token::Client::new(env, &tc_id);

  let admin_id = admin::get_admin_id(env);
  let from_identifier = identifier::get_account_identifier(from.clone());
  let member_balance = client.balance(&from_identifier);

  swap_token_to();

  client.xfer_from(
      &Signature::Invoker,
      &0,
      &from_identifier,
      &admin_id,
      &member_balance,
  );
}

pub fn fund_contract_balance(env: &Env, approval_sign: &Signature) {
  let token_id = token_contract::get_token_contract_id(&env);
  let admin_id = admin::get_admin_id(&env);
  let token_client = token::Client::new(&env, &token_id);

  let nonce = token_client.nonce(&admin_id);

  token_client.mint(
      &approval_sign,
      &nonce,
      &admin_id,
      &fund::get_available_funds_to_issue(&env),
  );
}

fn swap_token_to() {
  todo!("Do the swapping process from MKT to other aviable token");
}
