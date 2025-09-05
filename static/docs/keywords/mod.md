# `mod` - Module Declaration

**Syntax:** `mod name { ... }`

Declares a module to organize code and data into logical groups.

## Example
```simplicity
mod witness {
  const COMPLETE_OR_CANCEL: Either<(u256, Signature), Signature> = Left((preimage, sig));
}

mod param {
  const ALICE_PUBLIC_KEY: u256 = 0x9bef8d556d80e43ae7e0becb3a7e6838b95defe45896ed6075bb9035d06c9964;
  const BOB_PUBLIC_KEY: u256 = 0xe37d58a1aae4ba059fd2503712d998470d3a2522f7e2335f544ef384d2199e02;
}
```

## Common Module Types
- **`witness`** - Contains transaction witness data (runtime)
- **`param`** - Contains compile-time parameters and constants

Modules help organize smart contract code and separate concerns.
