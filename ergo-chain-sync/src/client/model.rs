use ergo_lib::chain::transaction::Transaction;
use ergo_lib::ergo_chain_types::Header;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockTransactions {
    #[serde_as(as = "serde_with::VecSkipError<_>")]
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
pub struct FullBlock {
    pub header: Header,
    pub block_transactions: BlockTransactions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiInfo {
    pub full_height: u32
}
