# `jet::subtract_256(a, b)` - 256-bit Subtraction

**Parameters:**
- `a: u256` - Minuend (number to subtract from)
- `b: u256` - Subtrahend (number to subtract)

**Returns:** `u256` - Difference (a - b)

Performs subtraction on two 256-bit unsigned integers.

## Usage
```simplicity
let difference: u256 = jet::subtract_256(total, amount);
let remaining: u256 = jet::subtract_256(balance, spend);
```

**Note:** Will fail if `b > a` (underflow protection).
