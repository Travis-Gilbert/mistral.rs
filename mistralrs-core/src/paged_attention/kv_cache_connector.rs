//! Optional external KV cache connector hooks for paged attention.
//!
//! The default connector is inert, preserving mistral.rs's current local-only
//! prefix cache behavior. Integrations can install a connector on
//! [`KVCacheManager`](super::kv_cache_manager::KVCacheManager) to observe local
//! events or hydrate externally stored blocks into freshly allocated slots.

use super::block_hash::{BlockHash, BlockHashWithGroupId};

/// Additive hook for external KV cache tiers.
pub trait KvCacheConnector: Send + Sync {
    /// Return how many leading block hashes are available externally.
    ///
    /// The manager only calls this after local prefix-cache lookup misses, and
    /// only for the still-contiguous prefix suffix that can be used safely.
    fn lookup_blocks(&self, _block_hashes: &[BlockHash], _group_ids: &[u32]) -> usize {
        0
    }

    /// Hydrate an externally found block into a freshly allocated physical slot.
    ///
    /// Returning `false` rejects the external hit and makes allocation fail,
    /// allowing the caller to retry without the connector path.
    fn load_block(
        &self,
        _block_hash: BlockHash,
        _group_ids: &[u32],
        _target_block_id: usize,
    ) -> bool {
        false
    }

    /// Observe locally stored full blocks.
    fn observe_store(
        &self,
        _block_hashes: &[BlockHash],
        _block_ids: &[usize],
        _num_cached_blocks: usize,
        _num_full_blocks: usize,
        _group_ids: &[u32],
    ) {
    }

    /// Offer a soon-to-be-reused cached block to the external tier.
    fn offer_evicted_block(&self, _block_hashes: &[BlockHashWithGroupId], _block_id: usize) {}
}

#[derive(Debug, Default)]
pub struct NoopKvCacheConnector;

impl KvCacheConnector for NoopKvCacheConnector {}
