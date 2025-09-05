# `Ctx8` - SHA-256 Hash Context

Represents a SHA-256 hashing context for incremental hash computation.

## Usage
```simplicity
fn sha2(string: u256) -> u256 {
  let hasher: Ctx8 = jet::sha_256_ctx_8_init();
  let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
  jet::sha_256_ctx_8_finalize(hasher)
}
```

Used with the SHA-256 jet functions for cryptographic hashing.
