# Simplicity Language Reference

## Keywords
- **`const`** - Declares a constant value
- **`fn`** - Declares a function
- **`let`** - Declares a local variable
- **`mod`** - Declares a module
- **`match`** - Pattern matching expression
- **`assert!`** - Assertion macro for conditions
- **`main`** - Entry point function

## Control Flow Types
- **`Either<L, R>`** - Sum type with `Left(L)` and `Right(R)` variants
- **`Option<T>`** - Optional type with `Some(T)` and `None` variants

## Primitive Types
- **`u256`** - 256-bit unsigned integer (Bitcoin addresses, hashes)
- **`u128`** - 128-bit unsigned integer
- **`u64`** - 64-bit unsigned integer
- **`u32`** - 32-bit unsigned integer
- **`u16`** - 16-bit unsigned integer
- **`u8`** - 8-bit unsigned integer

## Bitcoin-Specific Types
- **`Pubkey`** - Bitcoin public key (32 bytes)
- **`Signature`** - Digital signature (64 bytes)
- **`Height`** - Block height for timelock operations
- **`Ctx8`** - SHA-256 hash context

## Array Types
- **`[u8; 32]`** - 32-byte array (public keys, hashes)
- **`[u8; 64]`** - 64-byte array (signatures)

## Jet Functions

### Cryptographic Operations
- **`jet::sha_256_ctx_8_init()`** - Initialize SHA-256 context
- **`jet::sha_256_ctx_8_add_32(ctx, data)`** - Add 32 bytes to hash context
- **`jet::sha_256_ctx_8_finalize(ctx)`** - Finalize hash and return digest
- **`jet::bip_0340_verify((pubkey, msg), sig)`** - Verify BIP-340 Schnorr signature
- **`jet::sig_all_hash()`** - Get signature hash for all transaction inputs

### Comparison Operations
- **`jet::eq_256(a, b)`** - Compare two 256-bit values for equality
- **`jet::eq_128(a, b)`** - Compare two 128-bit values for equality
- **`jet::eq_64(a, b)`** - Compare two 64-bit values for equality
- **`jet::eq_32(a, b)`** - Compare two 32-bit values for equality

### Bitcoin Operations
- **`jet::check_lock_height(height)`** - Verify block height timelock
- **`jet::check_lock_time(time)`** - Verify time-based timelock
- **`jet::current_index()`** - Get current transaction input index
- **`jet::current_amount()`** - Get current input amount
- **`jet::current_asset()`** - Get current input asset ID
- **`jet::current_script_hash()`** - Get current script hash

### Arithmetic Operations
- **`jet::add_256(a, b)`** - Add two 256-bit values
- **`jet::subtract_256(a, b)`** - Subtract two 256-bit values
- **`jet::multiply_256(a, b)`** - Multiply two 256-bit values

### Bitwise Operations
- **`jet::and_256(a, b)`** - Bitwise AND on 256-bit values
- **`jet::or_256(a, b)`** - Bitwise OR on 256-bit values
- **`jet::xor_256(a, b)`** - Bitwise XOR on 256-bit values
- **`jet::not_256(a)`** - Bitwise NOT on 256-bit value
- **`jet::shift_left_256(value, bits)`** - Left shift 256-bit value
- **`jet::shift_right_256(value, bits)`** - Right shift 256-bit value

## Common Patterns

### Module Structure
```simplicity
mod witness {
  const DATA: u256 = 0x1234...;
}

mod param {
  const PUBLIC_KEY: u256 = 0xabcd...;
}
```

### Function Signatures
```simplicity
fn function_name(param: Type) -> ReturnType {
  // function body
}
```

### Pattern Matching
```simplicity
match value {
  Left(data) => { /* handle left case */ },
  Right(data) => { /* handle right case */ },
}
```

### Hash Verification
```simplicity
fn verify_hash(preimage: u256, expected: u256) {
  let hash = sha2(preimage);
  assert!(jet::eq_256(hash, expected));
}
```

### Signature Verification
```simplicity
fn verify_signature(pubkey: Pubkey, signature: Signature) {
  let msg = jet::sig_all_hash();
  jet::bip_0340_verify((pubkey, msg), signature);
}
```

## Tips
- Use hexadecimal literals with `0x` prefix for addresses and hashes
- Always verify signatures and hashes in smart contracts
- Use timelocks for escrow and payment channel contracts
- Pattern match on `Either` types for conditional logic
