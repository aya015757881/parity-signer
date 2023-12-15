use std::str::FromStr;
use parity_scale_codec::Encode;
use subxt::{
    config::{
        extrinsic_params::{BaseExtrinsicParams, BaseExtrinsicParamsBuilder, Era, ExtrinsicParams},
        polkadot::PlainTip,
        PolkadotConfig,
    },
    utils::{AccountId32, H256},
};

#[subxt::subxt(runtime_metadata_path = "for_tests/westend9111.scale")]
mod westend9111 {}

#[subxt::subxt(runtime_metadata_path = "for_tests/westend9122.scale")]
mod westend9122 {}

#[subxt::subxt(runtime_metadata_path = "for_tests/acala2012.scale")]
mod acala2012 {}

#[subxt::subxt(runtime_metadata_path = "for_tests/moonbase1802.scale")]
mod moonbase1802 {}

// This struct is needed as a way to add a `Compact`
// length in front of the encoded method payload.
#[derive(parity_scale_codec::Encode)]
struct Method {
    method: Vec<u8>,
}

fn westend_genesis() -> H256 {
    H256::from_str("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap()
}

// Encode `Call` and `Extensions` into a payload that UOS compatible.
fn encode_call_and_params<I, H, C: Encode, P: ExtrinsicParams<I, H>>(
    call: &C,
    params: &P,
) -> Vec<u8> {
    let call = call.encode();

    let call = Method { method: call };

    let mut extensions = vec![];
    params.encode_extra_to(&mut extensions);
    params.encode_additional_to(&mut extensions);
    let mut tx = call.encode();

    tx.extend_from_slice(extensions.as_slice());

    tx
}

#[test]
fn tr_0() {
    use westend9111::runtime_types::{
        pallet_balances::pallet::Call as BalancesCall, westend_runtime::Call,
    };

    let dest = AccountId32::from_str(
        "5DoW9HHuqSfpf55Ux5pLdJbHFWvbngeg8Ynhub9DrdtxmZeV"
    )
        .unwrap()
        .into();

    let call = BalancesCall::transfer {
        dest,
        value: 5_000_000_000_000,
    };

    let call = Call::Balances(call);
    
    let genesis_hash = westend_genesis();
    let block_hash =
        H256::from_str("80ed521e37056d43f4fcbdaf2a37faba951c35d9608dbff8fd2d4b5fd3fdbea3").unwrap();
    
    let params = BaseExtrinsicParams::<PolkadotConfig, PlainTip>::new(
        104000,
        24,
        0,
        genesis_hash,
        BaseExtrinsicParamsBuilder::new()
            .tip(PlainTip::new(100_000_000_000))
            .era(Era::Immortal, block_hash),
    );

    let tx = encode_call_and_params(&call, &params);
    let tx = hex::encode(&tx);

    println!("{}", tx);
}