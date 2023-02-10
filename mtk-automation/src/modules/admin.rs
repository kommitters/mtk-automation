pub mod admin {
    use soroban_sdk::Env;
    use soroban_auth::Identifier;
    use crate::modules::datakey::DataKey;

    pub fn get_admin_id(env: &Env) -> Identifier {
        let key = DataKey::AdminId;
        env.storage().get(key).unwrap().unwrap()
    }

    pub fn set_admin_id(env: &Env, account_id: &Identifier) {
        env.storage().set(DataKey::AdminId, account_id);
    }
}
