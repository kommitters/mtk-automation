// Module where you get and revoke members of the organization
pub mod member {
    use soroban_sdk::{vec, Env, Vec, AccountId, RawVal};
    use soroban_auth::Signature;
    use crate::contract_actions::datakey::DataKey;
    use crate::contract_actions::token_contract::token_contract;
    use crate::contract_actions::admin::admin;
    use crate::contract_actions::identifier_wrapper::identifier;
    use crate::contract_actions::token;

    const KEY: DataKey = DataKey::Members;

    pub fn add_member(env: &Env, account: AccountId) {
        let mut members = get_members(&env);
        members.push_back(account);
        env.storage().set(KEY, members);
    }
      
    pub fn revoke_membership(env: &Env, from: &AccountId) {
        let mut members: Vec<AccountId> = get_members(&env);
        
        let index;
      
        match members.first_index_of(from) {
            Some(i) => index = i,
            None => panic!("You are trying to remove an account that doesn't belong to your organization"),
        }
      
        members.remove(index);
        env.storage().set(KEY, members);

        bring_back_tokens_to_admin(&env, &from)
    }

    fn bring_back_tokens_to_admin(env: &Env, from: &AccountId){
        let tc_id = token_contract::get_token_contract_id(&env);
        let client = token::Client::new(&env, &tc_id);
      
        let admin_id = admin::get_admin_id(&env);
        let from_identifier = identifier::get_account_identifier(from.clone());
        let member_balance = client.balance(&from_identifier);
      
        client.xfer_from(
            &Signature::Invoker, 
            &0, 
            &from_identifier, 
            &admin_id,
            &member_balance
        );
        todo!("Swapping...");
    }
      
    pub fn get_members<T: soroban_sdk::TryFromVal<Env, RawVal> + soroban_sdk::IntoVal<Env, RawVal>>(
        e: &Env,
    ) -> Vec<T> {
        e.storage()
            .get(KEY)
            .unwrap_or(Ok(vec![e])) // if no members on vector
            .unwrap()
    }
}
