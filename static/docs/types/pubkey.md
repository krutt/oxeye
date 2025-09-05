# `Pubkey` - Bitcoin Public Key

Represents a Bitcoin public key (32 bytes) used for signature verification.

## Usage
```simplicity
const ALICE_PUBLIC_KEY: Pubkey = 0x9bef8d556d80e43ae7e0becb3a7e6838b95defe45896ed6075bb9035d06c9964;

fn verify_signature(pk: Pubkey, sig: Signature) {
  let msg: u256 = jet::sig_all_hash();
  jet::bip_0340_verify((pk, msg), sig);
}
```

Used with `jet::bip_0340_verify` for BIP-340 Schnorr signature verification.
