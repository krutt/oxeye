# `Either<L, R>` - Sum Type

Represents a value that can be one of two types: `Left(L)` or `Right(R)`.

## Variants
- **`Left(value)`** - Contains a value of type L
- **`Right(value)`** - Contains a value of type R

## Example
```simplicity
const CHOICE: Either<(u256, Signature), Signature> = Left((preimage, sig));

match CHOICE {
  Left(data: (u256, Signature)) => handle_complete(data),
  Right(sig: Signature) => handle_cancel(sig),
}
```

Perfect for conditional execution in smart contracts.
