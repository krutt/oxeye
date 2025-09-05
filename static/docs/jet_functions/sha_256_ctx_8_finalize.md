# `jet::sha_256_ctx_8_finalize(ctx)` - Finalize SHA-256 Hash

**Parameters:**
- `ctx: Ctx8` - Hash context to finalize

**Returns:** `u256` - Final SHA-256 hash digest

Finalizes a SHA-256 hash context and returns the computed hash.

## Usage
```simplicity
fn sha2(string: u256) -> u256 {
  let hasher: Ctx8 = jet::sha_256_ctx_8_init();
  let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
  jet::sha_256_ctx_8_finalize(hasher)
}

fn verify_preimage(preimage: u256, expected_hash: u256) {
  let hash: u256 = sha2(preimage);
  assert!(jet::eq_256(hash, expected_hash));
}
```

**Security:** Essential for hash-based proofs and preimage verification in smart contracts.
