pub mod member {
    use soroban_sdk::{vec, Env, Vec, AccountId, RawVal};
    use soroban_auth::Signature;
    use crate::modules::datakey::DataKey;
    use crate::modules::token_contract::token_contract;
    use crate::modules::admin::admin;
    use crate::modules::identifier_wrapper::identifier;
    use crate::modules::token;

    pub fn add_member(env: &Env, account: AccountId) {
        let mut members = get_members(&env);
      
        members.push_back(account);
      
        let key = DataKey::Members;
        env.storage().set(key, members);
    }
      
    pub fn revoke_membership(env: &Env, from: &AccountId) {
        let mut members: Vec<AccountId> = get_members(&env);
        
        let index;
      
        match members.first_index_of(from) {
            Some(i) => index = i,
            None => panic!("You are trying to remove an account that doesn't belong to your organization"),
        }
      
        members.remove(index);
      
        let key = DataKey::Members;
        env.storage().set(key, members);
      
        // Bring back it's TOKEN's to the admin
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
    }
      
    pub fn get_members<T: soroban_sdk::TryFromVal<Env, RawVal> + soroban_sdk::IntoVal<Env, RawVal>>(
        e: &Env,
    ) -> Vec<T> {
        let key = DataKey::Members;
        e.storage()
            .get(key)
            .unwrap_or(Ok(vec![e])) // if no members on vector
            .unwrap()
    }
}
