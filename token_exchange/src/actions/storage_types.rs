//! Module StorageTypes
//!
//! Module that defines the types allowed to store data in the contract
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Offer,
    SellTokenA, //Sell token amount storage
    BuyTokenA,  //Buy token amount storage
}
