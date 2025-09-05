# `Option<T>` - Optional Value Type

Represents a value that may or may not be present.

## Variants
- **`Some(value)`** - Contains a value of type T
- **`None`** - No value present

## Usage
```simplicity
let maybe_value: Option<u256> = Some(12345);

match maybe_value {
  Some(value) => use_value(value),
  None => handle_none_case(),
}
```

Essential for handling nullable values safely.
