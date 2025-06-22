#[cfg(test)]
extern crate std;

use crate::{StakingVault, StakingVaultClient};
use soroban_sdk::testutils::{Address as _, BytesN as _};
use soroban_sdk::{Address, BytesN, Env};

fn setup() -> (Env, Address, Address, Address, BytesN<32>) {
    let env = Env::default();
    let base_token_admin = Address::generate(&env);
    let vault_admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token_wasm_hash = BytesN::from_array(&env, &[1u8; 32]);
    (env, base_token_admin, vault_admin, user, token_wasm_hash)
}

#[test]
fn test_initialize_and_getters() {
    let (env, _, vault_admin, _, token_wasm_hash) = setup();
    let base_token = Address::generate(&env);
    let contract_id = env.register_contract(None, StakingVault);
    let client = StakingVaultClient::new(&env, &contract_id);

    client.initialize(&token_wasm_hash, &base_token, &vault_admin);

    assert_eq!(client.get_base_token(), base_token);
    assert_eq!(client.get_total_supply(), 0);
    assert!(client.get_vault_token().is_contract());
}

#[test]
fn test_deposit_and_withdraw() {
    let (env, _, vault_admin, user, token_wasm_hash) = setup();
    let base_token = Address::generate(&env);
    let contract_id = env.register_contract(None, StakingVault);
    let client = StakingVaultClient::new(&env, &contract_id);

    client.initialize(&token_wasm_hash, &base_token, &vault_admin);
    let vault_token = client.get_vault_token();

    let base_token_client = token::Client::new(&env, &base_token);
    let vault_token_client = token::Client::new(&env, &vault_token);

    base_token_client.mint(&user, &1000);
    assert_eq!(base_token_client.balance(&user), 1000);

    env.set_authorized(&user, true);
    client.deposit(&user, &500);

    assert_eq!(base_token_client.balance(&user), 500);
    assert_eq!(base_token_client.balance(&contract_id), 500);
    assert_eq!(vault_token_client.balance(&user), 500);
    assert_eq!(client.get_total_supply(), 500);

    client.withdraw(&user, &200);

    assert_eq!(base_token_client.balance(&user), 700);
    assert_eq!(vault_token_client.balance(&user), 300);
    assert_eq!(client.get_total_supply(), 300);
}
