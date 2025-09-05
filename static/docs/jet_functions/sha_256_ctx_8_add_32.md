# `jet::sha_256_ctx_8_add_32(ctx, data)` - Add Data to SHA-256 Context

**Parameters:**
- `ctx: Ctx8` - Current hash context
- `data: u256` - 32 bytes of data to add to hash

**Returns:** `Ctx8` - Updated hash context

Adds 32 bytes of data to an existing SHA-256 hash context.

## Usage
```simplicity
fn sha2(string: u256) -> u256 {
  let hasher: Ctx8 = jet::sha_256_ctx_8_init();
  let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
  jet::sha_256_ctx_8_finalize(hasher)
}
```

Use this function to incrementally build up a hash from multiple pieces of data.
