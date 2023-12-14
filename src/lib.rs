#![deny(rustdoc::broken_intra_doc_links)]

use parity_scale_codec::{Decode, DecodeAll, Encode};
use sp_core::H256;
use sp_runtime::generic::Era;


pub mod cards;
use cards::ParserCard;
pub mod method;

#[cfg(test)]
mod tests;


/// Struct to decode pre-determined extensions for transactions with `V12` and `V13` metadata
#[derive(Debug, Decode, Encode)]
struct ExtValues {
    era: Era,
    #[codec(compact)]
    nonce: u64,
    #[codec(compact)]
    tip: u128,
    metadata_version: u32,
    tx_version: u32,
    genesis_hash: H256,
    block_hash: H256,
}
