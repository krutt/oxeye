# `jet::bip_0340_verify((pubkey, msg), signature)` - Verify BIP-340 Signature

**Parameters:**
- `(pubkey, msg): (Pubkey, u256)` - Public key and message hash
- `signature: Signature` - Signature to verify

Verifies a BIP-340 Schnorr signature against a public key and message.

## Usage
```simplicity
fn checksig(pk: Pubkey, sig: Signature) {
  let msg: u256 = jet::sig_all_hash();
  jet::bip_0340_verify((pk, msg), sig);
}
```

**Security:** Essential for verifying transaction signatures in Bitcoin smart contracts.
