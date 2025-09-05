# `fn` - Function Declaration

**Syntax:** `fn name(param: Type) -> ReturnType { ... }`

Declares a function with parameters and return type.

## Example
```simplicity
fn checksig(pk: Pubkey, sig: Signature) {
  let msg: u256 = jet::sig_all_hash();
  jet::bip_0340_verify((pk, msg), sig);
}

fn sha2(string: u256) -> u256 {
  let hasher: Ctx8 = jet::sha_256_ctx_8_init();
  let hasher: Ctx8 = jet::sha_256_ctx_8_add_32(hasher, string);
  jet::sha_256_ctx_8_finalize(hasher)
}
```

Functions are the building blocks of Simplicity smart contracts.
