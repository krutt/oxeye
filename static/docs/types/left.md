# `Left(value)` - Either Variant

The left variant of `Either<L, R>` type, containing a value of type L.

## Usage
```simplicity
const CHOICE: Either<u256, Signature> = Left(12345);

match CHOICE {
  Left(number) => handle_number(number),
  Right(sig) => handle_signature(sig),
}
```

Used for conditional logic and union types in smart contracts.
