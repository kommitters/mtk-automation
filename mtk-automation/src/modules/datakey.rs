use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    OrgName,
    TokenId,
    AdminId,
    Rewards,
    Members,
    AllowedF
}
