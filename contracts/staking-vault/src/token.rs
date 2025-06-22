#![allow(unused)]
use soroban_sdk::{contractimport, xdr::ToXdr, Address, Bytes, BytesN, Env, FromVal, String};

contractimport!(file = "../../target/wasm32v1-none/release/token.wasm");

pub fn get_salt(e: &Env, base_token: &Address) -> BytesN<32> {
    let xdr = base_token.to_xdr(e);
    e.crypto().sha256(&xdr).into()
}

pub fn create_vault_token(
    e: &Env,
    token_wasm_hash: BytesN<32>,
    admin: &Address,
    salt: &BytesN<32>,
) -> Address {
    e.deployer().with_current_contract(salt.clone()).deploy_v2(
        token_wasm_hash,
        (
            e.current_contract_address(), // admin set to this contract
            7u32,                         // decimals
            String::from_val(e, &"Vault Token"),
            String::from_val(e, &"VToken"),
        ),
    )
}
