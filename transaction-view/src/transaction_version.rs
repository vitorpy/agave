/// A byte that represents the version of the transaction.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum TransactionVersion {
    #[default]
    Legacy = u8::MAX,
    V0 = 0,
    /// Transaction message version 1 (SIMD-0385 wire format).
    V1 = 1,
}
