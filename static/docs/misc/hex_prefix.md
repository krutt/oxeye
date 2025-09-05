# `0x` - Hexadecimal Literal Prefix

Prefix for hexadecimal number literals in Simplicity.

## Usage
```simplicity
const PUBLIC_KEY: u256 = 0x9bef8d556d80e43ae7e0becb3a7e6838b95defe45896ed6075bb9035d06c9964;
const HASH: u256 = 0x3034f1d855651180a6ae0ab3fb46c2b87501297e491e665a097f2f7d4ef5c835;
const SIGNATURE: [u8; 64] = 0x865b365eaa74e82bb25fbd401af5760d1af04168289f10cf26efc9e53f54b43c23761f59ffa6885f6d49fcdf556ddbfff0aabdafcc9cab568c9e82e5f826dd85;
```

**Common in Bitcoin:** Addresses, hashes, public keys, and signatures are typically expressed in hexadecimal.
