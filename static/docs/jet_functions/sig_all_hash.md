# `jet::sig_all_hash()` - Get Signature Hash

**Returns:** `u256` - Transaction signature hash

Returns the signature hash for all transaction inputs (SIGHASH_ALL).

## Usage
```simplicity
fn checksig(pk: Pubkey, sig: Signature) {
  let msg: u256 = jet::sig_all_hash();
  jet::bip_0340_verify((pk, msg), sig);
}
```

**Important:** This is the message that gets signed in Bitcoin transactions, covering all inputs and outputs.
