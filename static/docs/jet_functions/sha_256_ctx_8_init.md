# `jet::sha_256_ctx_8_init()` - Initialize SHA-256 Context

**Returns:** `Ctx8` - SHA-256 hash context

Initializes a new SHA-256 hashing context for incremental hashing.

## Usage
```simplicity
fn sha2(string: u256) -> u256 {
  let hasher: Ctx8 = jet::sha_256_ctx_8_init();
  let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
  jet::sha_256_ctx_8_finalize(hasher)
}
```

Part of the SHA-256 incremental hashing API along with `add_32` and `finalize`.
