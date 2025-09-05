# `const` - Constant Declaration

**Syntax:** `const NAME: Type = value;`

Declares a compile-time constant value that cannot be changed.

## Example
```simplicity
const PUBLIC_KEY: u256 = 0x9bef8d556d80e43ae7e0becb3a7e6838b95defe45896ed6075bb9035d06c9964;
const TIMEOUT: Height = 1000;
```

Constants are commonly used for:
- Bitcoin public keys and addresses
- Hash values and preimages  
- Timeout values for timelocks
- Configuration parameters
