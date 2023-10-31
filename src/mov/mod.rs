use anyhow::{anyhow, Ok, Result};
use move_core_types::language_storage::TypeTag;
use move_core_types::u256::U256;
use move_core_types::value::MoveValue;
use serde_json::json;
use shared_crypto::intent::Intent;
use std::f32::consts::E;
use std::path::Path;
use std::str::FromStr;
use sui::client_commands::{SuiClientCommandResult, SuiClientCommands};
use sui_json::MoveTypeLayout;
use sui_json::SuiJsonValue;
use sui_json_rpc_types::SuiExecutionStatus;
use sui_json_rpc_types::SuiTransactionBlockEffectsAPI;
use sui_keys::keystore::AccountKeystore;
use sui_keys::keystore::Keystore;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::wallet_context::WalletContext;
use sui_types::base_types::ObjectID;
use sui_types::crypto::SignatureScheme;
use sui_types::transaction::{
    SenderSignedData, Transaction, TransactionData, TransactionDataAPI,
};

use crate::misc;

pub mod multisig;
pub mod utils;

pub async fn init_wallet(
    config_path: &String,
    key: &String,
    mnemonic: &String,
) -> Result<WalletContext> {
    let from_path_str = Path::new(config_path);
    let mut wallet = WalletContext::new(from_path_str, None, None)
        .await
        .or(Err(anyhow!("Failed to get wallet context")))?;
    let active_env = wallet
        .config
        .get_active_env()
        .or(Err(anyhow!("Failed to get active env")));
    println!("active env: {:?}", active_env);

    match &mut wallet.config.keystore {
        Keystore::File(file) => {
            let mut ret: Result<SuiAddress, anyhow::Error> =
                Err(anyhow!("import key/mnemonic failed"));
            if key.len() > 0 {
                ret = file.import_from_keystore(key);
            } else if mnemonic.len() > 0 {
                ret = file.import_from_mnemonic(
                    mnemonic,
                    SignatureScheme::ED25519,
                    None,
                );
            }
            match ret {
                core::result::Result::Ok(_) => {
                    println!("import mnemonic success")
                }
                Err(_) => return Err(anyhow!("import mnemonic failed")),
            }
        }
        Keystore::InMem(_) => {}
    }

    for x in &wallet.get_addresses() {
        println!("address :{}", x.to_string());
    }

    let active_address = wallet
        .active_address()
        .or(Err(anyhow!("Failed to get active address")))?;
    println!("active address: {:?}", active_address);

    let total_balance = get_total_gas_balance(&wallet, &active_address).await?;
    println!("total balances: {}", total_balance);

    Ok(wallet)
}

pub async fn get_balance(wallet: &mut WalletContext) -> Result<u64> {
    let active_address = wallet
        .active_address()
        .or(Err(anyhow!("Failed to get active address")))?;
    let balances = wallet.gas_objects(active_address).await?;
    let mut total_balance = 0u64;
    for gas in balances {
        let v1 = gas.1;
        println!(" {} = {}", v1.object_id, gas.0);
        total_balance += gas.0;
    }

    Ok(total_balance)
}

pub async fn get_multi_balance(
    wallet: &mut WalletContext,
    multi_address: &String,
    gas_id: &String,
) -> Result<u64> {
    let active_address: SuiAddress = SuiAddress::from_str(multi_address)?;
    let gas_id = ObjectID::from_hex_literal(gas_id)?;
    let balances = wallet.gas_objects(active_address).await?;
    let mut total_balance = 0u64;
    for gas in balances {
        let v1 = gas.1;
        println!(" {} = {}", v1.object_id, gas.0);
        if gas_id == v1.object_id {
            total_balance += gas.0;
        }
    }

    Ok(total_balance)
}

pub async fn get_total_gas_balance(
    wallet: &WalletContext,
    address: &SuiAddress,
) -> Result<u64> {
    let balances = wallet.gas_objects(*address).await?;
    let mut total_balance = 0u64;
    for gas in balances {
        let v1 = gas.1;
        println!(" {} = {}", v1.object_id, gas.0);
        total_balance += gas.0;
    }

    Ok(total_balance)
}

pub fn pack_params(
    oracle_cap: &String,
    price_oracle: &String,
    pool_ids: &Vec<u8>,
    token_prices: &Vec<u64>,
) -> Result<Vec<SuiJsonValue>> {
    let oracle_cap = ObjectID::from_hex_literal(oracle_cap)?;
    let price_oracle = ObjectID::from_hex_literal(price_oracle)?;
    let clock = ObjectID::from_hex_literal(clock)?;

    let id_len = pool_ids.len();
    let price_len = token_prices.len();
    if id_len != price_len {
        return Err(anyhow!("prices error"));
    }

    let mut idxs = Vec::new();
    for i in 0..id_len {
        idxs.push(MoveValue::U8(pool_ids[i]));
    }
    let json_idxs = json!(idxs);

    let mut prices = Vec::new();
    for i in 0..price_len {
        prices.push(MoveValue::U256(token_prices[i].into()));
    }
    let prices_val = MoveValue::Vector(prices);
    let prices_bytes = prices_val.simple_serialize().unwrap();

    let ts = misc::get_timestamp();
    let tss = vec![MoveValue::U64(ts); id_len];
    //let json_tss = json!(tss);
    let tss_val = MoveValue::Vector(tss);
    let tss_bytes = tss_val.simple_serialize().unwrap();

    Ok(vec![
        SuiJsonValue::from_object_id(oracle_cap),
        SuiJsonValue::from_object_id(price_oracle),
        SuiJsonValue::from_object_id(clock),
        SuiJsonValue::new(json_idxs)?,
        SuiJsonValue::from_bcs_bytes(
            Some(&MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U256))),
            &prices_bytes,
        )?,
        SuiJsonValue::from_bcs_bytes(
            Some(&MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U64))),
            &tss_bytes,
        )?,
    ])
}

pub async fn call(
    wallet: &mut WalletContext,
    package_id: &String,
    module: &String,
    function: &String,
    gas_budget: u64,
    params: Vec<SuiJsonValue>,
) -> Result<SuiClientCommandResult> {
    let package_id = ObjectID::from_hex_literal(package_id)?;

    SuiClientCommands::Call {
        package: package_id,
        module: module.into(),
        function: function.into(),
        type_args: vec![],
        args: params,
        gas: None,
        gas_budget: gas_budget,
        serialize_unsigned_transaction: false,
        serialize_signed_transaction: false,
    }
    .execute(wallet)
    .await
}

pub async fn multi_call(
    wallet: &mut WalletContext,
    package_id: &String,
    module: &String,
    function: &String,
    gas_id: &String,
    gas_budget: u64,
    params: Vec<SuiJsonValue>,
    pubkeys: &Vec<String>,
    weights: &Vec<u8>,
    threshold: u16,
) -> Result<SuiClientCommandResult> {
    let package = ObjectID::from_hex_literal(package_id)?;
    let gas = ObjectID::from_hex_literal(gas_id)?;
    let tx_data = construct_move_call_transaction(
        wallet,
        package,
        &module,
        &function,
        Some(gas),
        gas_budget,
        params,
        &pubkeys,
        &weights,
        threshold,
    )
    .await?;
    let signature = wallet.config.keystore.sign_secure(
        &wallet.config.active_address.unwrap(),
        &tx_data,
        Intent::sui_transaction(),
    )?;
    let sigs = vec![signature];
    let gen_sig = match self::multisig::multisig_combine_partialsig(
        sigs, pubkeys, weights, threshold,
    ) {
        core::result::Result::Ok(sig) => sig,
        Err(e) => return Err(e),
    };
    let gen_sigs = vec![gen_sig];
    let verified = Transaction::from_generic_sig_data(
        tx_data,
        Intent::sui_transaction(),
        gen_sigs,
    )
    .verify()?;

    let response = wallet.execute_transaction_may_fail(verified).await?;
    Ok(SuiClientCommandResult::ExecuteSignedTx(response))
}

async fn construct_move_call_transaction(
    wallet: &mut WalletContext,
    package: ObjectID,
    module: &str,
    function: &str,
    gas: Option<ObjectID>,
    gas_budget: u64,
    args: Vec<SuiJsonValue>,
    pubkeys: &Vec<String>,
    weights: &Vec<u8>,
    threshold: u16,
) -> Result<TransactionData, anyhow::Error> {
    let multi_address = self::multisig::multisig_address(
        pubkeys.clone(),
        weights.clone(),
        threshold,
    )?;
    let client = wallet.get_client().await?;
    client
        .transaction_builder()
        .move_call(
            multi_address,
            package,
            module,
            function,
            vec![],
            args,
            gas,
            gas_budget,
        )
        .await
}
