# `assert!` - Assertion Macro

**Syntax:** `assert!(condition);`

Asserts that a boolean condition is true. If false, the contract execution fails.

## Example
```simplicity
fn verify_hash(preimage: u256, expected: u256) {
  let hash: u256 = sha2(preimage);
  assert!(jet::eq_256(hash, expected));
}

fn verify_amount(amount: u64, minimum: u64) {
  assert!(jet::ge_64(amount, minimum));
}
```

## Usage in Smart Contracts
- **Hash verification** - Ensure preimages match expected hashes
- **Value validation** - Check amounts, timeouts, and other constraints
- **Security checks** - Validate critical contract conditions

**Critical:** Failed assertions terminate contract execution, making them essential for security.
