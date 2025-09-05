# `None` - Option Variant

The variant of `Option<T>` that represents no value.

## Usage
```simplicity
let no_value: Option<u256> = None;

match no_value {
  Some(v) => handle_value(v),
  None => handle_absence(),
}
```

Represents the absence of a value in a safe, explicit way.
