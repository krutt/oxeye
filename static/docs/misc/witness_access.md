# `witness::` - Witness Module Access

Access pattern for the witness module, which contains transaction witness data.

## Usage
```simplicity
mod witness {
  const COMPLETE_OR_CANCEL: Either<(u256, Signature), Signature> = ...;
}

fn main() {
  match witness::COMPLETE_OR_CANCEL {
    Left(data) => handle_complete(data),
    Right(sig) => handle_cancel(sig),
  }
}
```

Witness data is provided at transaction execution time.
