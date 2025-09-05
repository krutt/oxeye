# `jet::check_lock_height(height)` - Verify Block Height Timelock

**Parameters:**
- `height: Height` - Required block height

Verifies that the current block height meets or exceeds the specified timelock height.

## Usage
```simplicity
fn cancel_spend(sender_sig: Signature) {
  let timeout: Height = 1000;
  jet::check_lock_height(timeout);
  // ... rest of cancellation logic
}
```

**Use Cases:**
- Escrow contracts with time-based cancellation
- Payment channels with timeout mechanisms
- Time-delayed transactions
