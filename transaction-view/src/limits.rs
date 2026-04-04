//! Serialized transaction size limits ([SIMD-0296], [SIMD-0385]).
//!
//! [SIMD-0296]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0296-larger-transactions.md
//! [SIMD-0385]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0385-transaction-v1.md

pub use solana_message::v1::MAX_TRANSACTION_SIZE;
use solana_packet::PACKET_DATA_SIZE;

/// Maximum serialized size for legacy and v0 transactions (unchanged by SIMD-0296).
pub const MAX_LEGACY_OR_V0_TRANSACTION_SIZE: usize = PACKET_DATA_SIZE;
