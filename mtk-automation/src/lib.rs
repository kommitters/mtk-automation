#![no_std]

use soroban_sdk::{contractimpl, contracttype, vec, Env, Symbol, Vec, BytesN, AccountId, BigInt, RawVal, Map};


pub struct OrganizationContract;

pub trait OrganizationContractTrait {
    fn initialize(
        e: Env
    );
}

#[contractimpl]
impl OrganizationContractTrait for OrganizationContract {
    fn initialize(
        env: Env
    ){

    }
}

