use anyhow;
use base64_light::*;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{
    EncodeDecodeBase64, PublicKey, Signature, ToFromBytes,
};
use sui_types::multisig::{
    MultiSig, MultiSigPublicKey, ThresholdUnit, WeightUnit,
};
use sui_types::signature::GenericSignature;

pub fn multisig_address(
    pubkeys: Vec<String>,
    weights: Vec<u8>,
    threshold: u16,
) -> Result<SuiAddress, anyhow::Error> {
    let threshold_unit: ThresholdUnit = threshold;
    let weight_units: Vec<WeightUnit> = weights;
    let mut pks: Vec<PublicKey> = Vec::new();

    for key in pubkeys.iter() {
        match PublicKey::decode_base64(key) {
            Ok(pk) => pks.push(pk),
            Err(_e) => return Err(anyhow::anyhow!("Invalid public key")),
        }
    }

    let multisig_pk =
        MultiSigPublicKey::new(pks, weight_units, threshold_unit)?;
    let address: SuiAddress = (&multisig_pk).into();

    Ok(address)
}

pub fn multisig_combine_partialsig(
    sigs: Vec<Signature>,
    pubkeys: &Vec<String>,
    weights: &Vec<u8>,
    threshold: u16,
) -> Result<GenericSignature, anyhow::Error> {
    let threshold_unit: ThresholdUnit = threshold;
    let weight_units: Vec<WeightUnit> = weights.clone();
    let mut pks: Vec<PublicKey> = Vec::new();

    for key in pubkeys.iter() {
        match PublicKey::decode_base64(key) {
            Ok(pk) => pks.push(pk),
            Err(_e) => return Err(anyhow::anyhow!("Invalid public key")),
        }
    }

    let multisig_pk =
        MultiSigPublicKey::new(pks, weight_units, threshold_unit)?;
    let multisig = MultiSig::combine(sigs, multisig_pk)?;
    let generic_sig: GenericSignature = multisig.into();

    //println!("MultiSig serialized: {:?}", generic_sig.encode_base64());
    Ok(generic_sig)
}
