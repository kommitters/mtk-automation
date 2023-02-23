//! Module Offer
//! 
//! Represents an offer managed by the TokenTrade contract.
//! If a seller wants to sell 1000 XLM for 100 USDC the `sell_price` would be 1000
//! and `buy_price` would be 100 (or 100 and 10, or any other pair of integers
//! in 10:1 ratio).

use soroban_sdk::{contracttype, Address, BytesN};
#[derive(Clone)]
#[contracttype]
pub struct Offer {
    // Owner of this offer. Sells sell_token to get buy_token.
    pub seller: Address,
    pub sell_token: BytesN<32>,
    pub buy_token: BytesN<32>,
    // Seller-defined price of the sell token in arbitrary units.
    pub sell_price: u32,
    // Seller-defined price of the buy token in arbitrary units.
    pub buy_price: u32,
}