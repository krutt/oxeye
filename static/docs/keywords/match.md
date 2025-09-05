# `match` - Pattern Matching

**Syntax:** `match expression { pattern => body, ... }`

Pattern matching for handling different cases, especially with `Either` and `Option` types.

## Example
```simplicity
match witness::COMPLETE_OR_CANCEL {
  Left(preimage_and_sig: (u256, Signature)) => {
    let (preimage, sig): (u256, Signature) = preimage_and_sig;
    complete_spend(preimage, sig);
  },
  Right(sender_sig: Signature) => cancel_spend(sender_sig),
}
```

Essential for conditional logic in smart contracts.
