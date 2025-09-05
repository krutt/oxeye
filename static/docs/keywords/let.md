# `let` - Variable Declaration

**Syntax:** `let name: Type = value;`

Declares a local variable with an explicit type annotation.

## Example
```simplicity
let msg: u256 = jet::sig_all_hash();
let hasher: Ctx8 = jet::sha_256_ctx_8_init();
let (preimage, sig): (u256, Signature) = data;
```

Variables are immutable by default in Simplicity, promoting safety in smart contracts.
