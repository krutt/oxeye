# `param::` - Parameter Module Access

Access pattern for the parameter module, which contains compile-time constants.

## Usage
```simplicity
mod param {
  const ALICE_PUBLIC_KEY: u256 = 0x9bef8d...;
  const BOB_PUBLIC_KEY: u256 = 0xe37d58...;
}

fn verify_alice_signature(sig: Signature) {
  let pk: Pubkey = param::ALICE_PUBLIC_KEY;
  checksig(pk, sig);
}
```

Parameters are fixed at contract compilation time.
