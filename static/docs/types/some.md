# `Some(value)` - Option Variant

The variant of `Option<T>` that contains a value.

## Usage
```simplicity
let has_value: Option<u256> = Some(12345);

match has_value {
  Some(v) => println!("Value: {}", v),
  None => println!("No value"),
}
```

Used to wrap values in the Option type.
