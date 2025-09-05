# `Signature` - Digital Signature

Represents a digital signature (64 bytes) for cryptographic verification.

## Usage
```simplicity
fn checksig(pk: Pubkey, sig: Signature) {
  let msg: u256 = jet::sig_all_hash();
  jet::bip_0340_verify((pk, msg), sig);
}
```

Signatures are verified using `jet::bip_0340_verify` with the corresponding public key.
