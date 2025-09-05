# `Right(value)` - Either Variant

The right variant of `Either<L, R>` type, containing a value of type R.

## Usage
```simplicity
const CHOICE: Either<u256, Signature> = Right(signature);

match CHOICE {
  Left(number) => handle_number(number),
  Right(sig) => handle_signature(sig),
}
```

Used for conditional logic and alternative paths in smart contracts.
