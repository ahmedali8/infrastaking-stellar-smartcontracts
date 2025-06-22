#![no_std]

mod token;

use soroban_sdk::{
    contract, contractimpl, contractmeta, Address, BytesN, ConversionError, Env, TryFromVal, Val,
};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum VaultKey {
    BaseToken = 0,
    VaultToken = 1,
    Admin = 2,
    TotalVaultSupply = 3,
    IsInitialized = 4,
}

impl TryFromVal<Env, VaultKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &VaultKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

// Metadata that is added on to the WASM custom section
contractmeta!(key = "Description", val = "Staking Vault");

#[contract]
pub struct StakingVault;

#[contractimpl]
impl StakingVault {
    pub fn __constructor(e: Env, token_wasm_hash: BytesN<32>, base_token: Address, admin: Address) {
        if e.storage().instance().has(&VaultKey::IsInitialized) {
            panic!("Already initialized");
        }

        let salt = token::get_salt(&e, &base_token);
        let vtoken = token::create_vault_token(&e, token_wasm_hash, &admin, &salt);

        e.storage()
            .instance()
            .set(&VaultKey::BaseToken, &base_token);
        e.storage().instance().set(&VaultKey::VaultToken, &vtoken);
        e.storage().instance().set(&VaultKey::Admin, &admin);
        e.storage()
            .instance()
            .set(&VaultKey::TotalVaultSupply, &0i128);
        e.storage().instance().set(&VaultKey::IsInitialized, &true);
    }

    pub fn deposit(e: Env, from: Address, amount: i128) {
        from.require_auth();

        let base_token: Address = e.storage().instance().get(&VaultKey::BaseToken).unwrap();
        let vtoken: Address = e.storage().instance().get(&VaultKey::VaultToken).unwrap();
        let mut total_supply: i128 = e
            .storage()
            .instance()
            .get(&VaultKey::TotalVaultSupply)
            .unwrap_or(0);

        token::Client::new(&e, &base_token).transfer(&from, &e.current_contract_address(), &amount);
        token::Client::new(&e, &vtoken).mint(&from, &amount);

        total_supply += amount;
        e.storage()
            .instance()
            .set(&VaultKey::TotalVaultSupply, &total_supply);
    }

    pub fn withdraw(e: Env, to: Address, amount: i128) {
        to.require_auth();

        let base_token: Address = e.storage().instance().get(&VaultKey::BaseToken).unwrap();
        let vtoken: Address = e.storage().instance().get(&VaultKey::VaultToken).unwrap();
        let mut total_supply: i128 = e
            .storage()
            .instance()
            .get(&VaultKey::TotalVaultSupply)
            .unwrap_or(0);

        token::Client::new(&e, &vtoken).burn(&to, &amount);
        token::Client::new(&e, &base_token).transfer(&e.current_contract_address(), &to, &amount);

        total_supply -= amount;
        e.storage()
            .instance()
            .set(&VaultKey::TotalVaultSupply, &total_supply);
    }

    pub fn get_base_token(e: Env) -> Address {
        e.storage().instance().get(&VaultKey::BaseToken).unwrap()
    }

    pub fn get_vault_token(e: Env) -> Address {
        e.storage().instance().get(&VaultKey::VaultToken).unwrap()
    }

    pub fn get_total_supply(e: Env) -> i128 {
        e.storage()
            .instance()
            .get(&VaultKey::TotalVaultSupply)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
