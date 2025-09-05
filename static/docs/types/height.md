# `Height` - Block Height Type

Represents a Bitcoin block height for timelock operations.

## Usage
```simplicity
const TIMEOUT: Height = 1000;

fn check_timeout() {
  let timeout: Height = 800000;
  jet::check_lock_height(timeout);
}
```

Used with `jet::check_lock_height()` for implementing time-based contract conditions.
