#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, BytesN, Env, IntoVal, Symbol,
};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(
        e,
        &e.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    )
}

fn create_staking_vault_contract<'a>(
    e: &Env,
    token_wasm_hash: &BytesN<32>,
    base_token: &Address,
    admin: &Address,
) -> Address {
    let vault_address = e.register(crate::StakingVault {}, ());

    // Initialize the contract after registration using invoke_contract
    e.invoke_contract::<()>(
        &vault_address,
        &soroban_sdk::symbol_short!("init"),
        soroban_sdk::vec![
            e,
            token_wasm_hash.into_val(e),
            base_token.into_val(e),
            admin.into_val(e)
        ],
    );

    vault_address
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
    let vault_token_address: Address = e.invoke_contract(
        &vault,
        &Symbol::new(&e, "get_vault_token"),
        soroban_sdk::vec![&e],
    );
    let vault_token = token::Client::new(&e, &vault_token_address);

    // Mint base tokens to users
    base_token.mint(&user1, &1000);
    base_token.mint(&user2, &500);

    // Verify initial balances
    assert_eq!(base_token.balance(&user1), 1000);
    assert_eq!(base_token.balance(&user2), 500);
    assert_eq!(vault_token.balance(&user1), 0);
    assert_eq!(vault_token.balance(&user2), 0);
    let total_supply: i128 = e.invoke_contract(
        &vault,
        &Symbol::new(&e, "get_total_supply"),
        soroban_sdk::vec![&e],
    );
    assert_eq!(total_supply, 0);

    // Test first deposit by user1
    e.invoke_contract::<()>(
        &vault,
        &Symbol::new(&e, "deposit"),
        soroban_sdk::vec![&e, user1.into_val(&e), 200i128.into_val(&e)],
    );
}

// // Verify auth for deposit
// assert_eq!(
//     e.auths(),
//     std::vec![(
//         user1.clone(),
//         AuthorizedInvocation {
//             function: AuthorizedFunction::Contract((
//                 vault.address.clone(),
//                 symbol_short!("deposit"),
//                 (&user1, 200_i128).into_val(&e)
//             )),
//             sub_invocations: std::vec![AuthorizedInvocation {
//                 function: AuthorizedFunction::Contract((
//                     base_token.address.clone(),
//                     symbol_short!("transfer"),
//                     (&user1, &vault.address, 200_i128).into_val(&e)
//                 )),
//                 sub_invocations: std::vec![]
//             }]
//         }
//     )]
// );

// // Verify balances after first deposit
// assert_eq!(base_token.balance(&user1), 800);
// assert_eq!(base_token.balance(&vault.address), 200);
// assert_eq!(vault_token.balance(&user1), 200);
// assert_eq!(vault.get_total_supply(), 200);

// // Test second deposit by user2
// vault.deposit(&user2, &300);

// // Verify balances after second deposit
// assert_eq!(base_token.balance(&user2), 200);
// assert_eq!(base_token.balance(&vault.address), 500);
// assert_eq!(vault_token.balance(&user2), 300);
// assert_eq!(vault.get_total_supply(), 500);

// // Test partial withdrawal by user1
// vault.withdraw(&user1, &100);

// // Verify auth for withdrawal
// assert_eq!(
//     e.auths(),
//     std::vec![(
//         user1.clone(),
//         AuthorizedInvocation {
//             function: AuthorizedFunction::Contract((
//                 vault.address.clone(),
//                 symbol_short!("withdraw"),
//                 (&user1, 100_i128).into_val(&e)
//             )),
//             sub_invocations: std::vec![AuthorizedInvocation {
//                 function: AuthorizedFunction::Contract((
//                     vault_token.address.clone(),
//                     symbol_short!("burn"),
//                     (&user1, 100_i128).into_val(&e)
//                 )),
//                 sub_invocations: std::vec![]
//             }]
//         }
//     )]
// );

// // Verify balances after withdrawal
// assert_eq!(base_token.balance(&user1), 900);
// assert_eq!(base_token.balance(&vault.address), 400);
// assert_eq!(vault_token.balance(&user1), 100);
// assert_eq!(vault.get_total_supply(), 400);

// // Test complete withdrawal by user2
// e.budget().reset_unlimited();
// vault.withdraw(&user2, &300);

// // Verify final balances
// assert_eq!(base_token.balance(&user2), 500);
// assert_eq!(base_token.balance(&vault.address), 100);
// assert_eq!(vault_token.balance(&user2), 0);
// assert_eq!(vault.get_total_supply(), 100);

// // Test complete withdrawal by user1 (remaining balance)
// vault.withdraw(&user1, &100);

// // Verify all balances are back to expected state
// assert_eq!(base_token.balance(&user1), 1000);
// assert_eq!(base_token.balance(&user2), 500);
// assert_eq!(base_token.balance(&vault.address), 0);
// assert_eq!(vault_token.balance(&user1), 0);
// assert_eq!(vault_token.balance(&user2), 0);
// assert_eq!(vault.get_total_supply(), 0);

// // Verify contract addresses
// assert_eq!(vault.get_base_token(), base_token.address);
// assert_eq!(vault.get_vault_token(), vault_token.address);
