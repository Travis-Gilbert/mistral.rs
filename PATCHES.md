# Travis-Gilbert patch stack

## KV cache connector seam

- Issue-first proposal: <https://github.com/EricLBuehler/mistral.rs/issues/2308>
- Reason: expose an additive, no-op-by-default seam around the paged-attention block-pool prefix-cache flow so Theorem can attach RustyRed KV segment tiers without hard-forking block management.
- Upstream disposition: proposed upstream in issue #2308; local fork carries the patch until the interface is accepted, revised, or rejected.
- Diff summary: add `KvCacheConnector`/`NoopKvCacheConnector`, let `KVCacheManager` install a connector, report external contiguous prefix hits, hydrate those hits into freshly allocated blocks, observe local stores, and offer evicted cached blocks before their hash metadata is cleared.
