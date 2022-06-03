use near_chain::ChainStore;
use near_chain::ChainStoreAccess;
use near_primitives::account::id::AccountId;
use near_primitives::block::Block;
use near_primitives::transaction::SignedTransaction;
use near_primitives::types::ShardId;

/// Returns a list of transactions found in the block.
pub fn tx_dump(
    chain_store: &mut ChainStore,
    block: &Block,
    _select_account_ids: Option<&Vec<AccountId>>,
) -> Vec<SignedTransaction> {
    let chunks = block.chunks();
    let res = vec![];
    for (shard_id, chunk_header) in chunks.iter().enumerate() {
        let shard_id = shard_id as ShardId;
        println!("[{:?}] -- {:?}", shard_id, chain_store.get_chunk(&chunk_header.chunk_hash()).unwrap().transactions());
        res.extend(chain_store.get_chunk(&chunk_header.chunk_hash()).unwrap().transactions().to_vec());
    }
    return res;
}

// #[cfg(test)]
// mod test {
// }
