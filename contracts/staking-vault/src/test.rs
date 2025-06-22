#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, IntoVal, Symbol};

// Create a client wrapper for StakingVault
pub struct StakingVaultClient<'a> {
    env: &'a Env,
    address: Address,
}

impl<'a> StakingVaultClient<'a> {
    pub fn new(env: &'a Env, address: &Address) -> Self {
        Self {
            env,
            address: address.clone(),
        }
    }

    pub fn init(&self, token_wasm_hash: &BytesN<32>, base_token: &Address, admin: &Address) {
        self.env.invoke_contract::<()>(
            &self.address,
            &Symbol::new(self.env, "init"),
            soroban_sdk::vec![
                self.env,
                token_wasm_hash.into_val(self.env),
                base_token.into_val(self.env),
                admin.into_val(self.env)
            ],
        )
    }

    pub fn deposit(&self, from: &Address, amount: &i128) {
        self.env.invoke_contract::<()>(
            &self.address,
            &Symbol::new(self.env, "deposit"),
            soroban_sdk::vec![self.env, from.into_val(self.env), amount.into_val(self.env)],
        )
    }

    pub fn withdraw(&self, to: &Address, amount: &i128) {
        self.env.invoke_contract::<()>(
            &self.address,
            &Symbol::new(self.env, "withdraw"),
            soroban_sdk::vec![self.env, to.into_val(self.env), amount.into_val(self.env)],
        )
    }

    pub fn get_vault_token(&self) -> Address {
        self.env.invoke_contract(
            &self.address,
            &Symbol::new(self.env, "get_vault_token"),
            soroban_sdk::vec![self.env],
        )
    }

    pub fn get_base_token(&self) -> Address {
        self.env.invoke_contract(
            &self.address,
            &Symbol::new(self.env, "get_base_token"),
            soroban_sdk::vec![self.env],
        )
    }

    pub fn get_total_supply(&self) -> i128 {
        self.env.invoke_contract(
            &self.address,
            &Symbol::new(self.env, "get_total_supply"),
            soroban_sdk::vec![self.env],
        )
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(
        e,
        &e.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    )
}

fn create_staking_vault_contract<'a>(
    e: &'a Env,
    token_wasm_hash: &BytesN<32>,
    base_token: &Address,
    admin: &Address,
) -> StakingVaultClient<'a> {
    let vault_address = e.register(crate::StakingVault {}, ());
    let vault = StakingVaultClient::new(e, &vault_address);
    vault.init(token_wasm_hash, base_token, admin);
    vault
}

fn install_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/token.wasm");
    e.deployer().upload_contract_wasm(WASM)
}

#[test]
fn test_staking_vault_complete_flow() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    // Create base token contract
    let base_token = create_token_contract(&e, &admin);

    // Create staking vault contract
    let vault =
        create_staking_vault_contract(&e, &install_token_wasm(&e), &base_token.address, &admin);

    // Get vault token client
    let vault_token_address = vault.get_vault_token();
    let vault_token = token::Client::new(&e, &vault_token_address);

    // Mint base tokens to users
    base_token.mint(&user1, &1000);
    base_token.mint(&user2, &500);

    // Verify initial balances
    assert_eq!(base_token.balance(&user1), 1000);
    assert_eq!(base_token.balance(&user2), 500);
    assert_eq!(vault_token.balance(&user1), 0);
    assert_eq!(vault_token.balance(&user2), 0);
    assert_eq!(vault.get_total_supply(), 0);

    // Test first deposit by user1
    vault.deposit(&user1, &200);

    // Verify balances after first deposit
    assert_eq!(base_token.balance(&user1), 800);
    assert_eq!(base_token.balance(vault.address()), 200);
    assert_eq!(vault_token.balance(&user1), 200);
    assert_eq!(vault.get_total_supply(), 200);

    // Test second deposit by user2
    vault.deposit(&user2, &300);

    // Verify balances after second deposit
    assert_eq!(base_token.balance(&user2), 200);
    assert_eq!(base_token.balance(&vault.address), 500);
    assert_eq!(vault_token.balance(&user2), 300);
    assert_eq!(vault.get_total_supply(), 500);

    // Test partial withdrawal by user1
    vault.withdraw(&user1, &100);

    // Verify balances after withdrawal
    assert_eq!(base_token.balance(&user1), 900);
    assert_eq!(base_token.balance(&vault.address), 400);
    assert_eq!(vault_token.balance(&user1), 100);
    assert_eq!(vault.get_total_supply(), 400);

    // Test complete withdrawal by user2
    e.cost_estimate().budget().reset_unlimited();
    vault.withdraw(&user2, &300);

    // Verify final balances
    assert_eq!(base_token.balance(&user2), 500);
    assert_eq!(base_token.balance(&vault.address), 100);
    assert_eq!(vault_token.balance(&user2), 0);
    assert_eq!(vault.get_total_supply(), 100);

    // Test complete withdrawal by user1 (remaining balance)
    vault.withdraw(&user1, &100);

    // Verify all balances are back to expected state
    assert_eq!(base_token.balance(&user1), 1000);
    assert_eq!(base_token.balance(&user2), 500);
    assert_eq!(base_token.balance(&vault.address), 0);
    assert_eq!(vault_token.balance(&user1), 0);
    assert_eq!(vault_token.balance(&user2), 0);
    assert_eq!(vault.get_total_supply(), 0);

    // Verify contract addresses
    assert_eq!(vault.get_base_token(), base_token.address);
    assert_eq!(vault.get_vault_token(), vault_token_address);

    // Verify the vault token is valid and callable
    assert!(vault_token.balance(&user1) >= 0); // This proves the address is valid
}
