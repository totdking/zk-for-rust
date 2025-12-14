# Caesar Cipher Decryption

## Overview

The **Caesar cipher** is one of the simplest and oldest encryption techniques. It works by shifting each letter in the plaintext by a fixed number of positions in the alphabet. For example, with a shift of 3, 'A' becomes 'D', 'B' becomes 'E', and so on.

## How It Works

### Encryption
- Each letter is shifted forward by `k` positions in the alphabet
- Example: "HELLO" with shift 3 → "KHOOR"

### Decryption (This Implementation)
- This brute-force implementation tries all 26 possible shifts (0-25)
- For each shift, it shifts each letter backward by that amount
- The correct decryption will be the one that produces readable text

## Important Limitation: Single Encryption Only

⚠️ **This implementation works ONLY for text encrypted ONCE with a Caesar cipher.**

If the text has been encrypted **multiple times** (e.g., shift by 5, then shift by 7, then shift by 3), this simple brute-force approach will **NOT** work correctly. 

### Why Multiple Encryptions Break This Approach

When text is encrypted multiple times:
- First encryption: shift by k₁
- Second encryption: shift by k₂
- Third encryption: shift by k₃
- ...and so on

To decrypt, you would need to know:
1. **How many times** it was encrypted (let's call this `n`)
2. Try all possible combinations of `n` shifts

## Time & Space Complexity

### Single Encryption (Current Implementation)
- **Time Complexity**: `O(26 × m)` where `m` is the length of the ciphertext
  - We try 26 shifts, and for each shift, we process `m` characters
  - Simplifies to `O(m)` since 26 is a constant
- **Space Complexity**: `O(m)` for storing the decrypted string

### Multiple Encryptions (k times)
If the text was encrypted `k` times and you don't know the shifts:

- **Time Complexity**: `O(26^k × m)`
  - You need to try all combinations of `k` shifts
  - For each combination, process `m` characters
  - This is **exponential time**, not factorial
  - Example: 2 encryptions = 26² = 676 combinations, 3 encryptions = 26³ = 17,576 combinations
  
- **Space Complexity**: `O(m)` (still just storing one decrypted string at a time)

### Note on Complexity
The complexity is **exponential** `O(26^k)`, **not factorial** `O(k!)`. 
- Factorial would be if we were trying different orderings of operations
- Here we're trying all combinations of values, which is exponential

## Example Usage

```rust
ceaser_cipher("ITQZ FUXF BMPPXQ MFFUFGPQ");
// Output includes all 26 shifts, one of which will be:
// Shift 12: "WHEN TILT PADDLE ATTITUDE"
```

## Decrypted Result

For the ciphertext `"ITQZ FUXF BMPPXQ MFFUFGPQ"`:
- **Shift 12** produces: `"WHEN TILT PADDLE ATTITUDE"`
- This is the correct decryption!
