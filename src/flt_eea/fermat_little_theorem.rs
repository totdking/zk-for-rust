//! Standard "BigInt" style arithmetic for demonstration.
//! In production, use `crypto-bigint` or `num-bigint`.

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 { return 0; }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        // exp = exp / 2
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}

/// 1. Fermat's Little Theorem Inverse
/// RESTRICTION: Modulus `p` MUST be Prime.
fn inverse_fermat(a: i128, p: i128) -> Result<i128, &'static str> {
    if a != 1 {
        if p % a == 0 {
            return Err("Cannot pass in a factor of the prime modulus field");
        }
    }
    if a <= 0 || a >= p {
        // In a real field, we strictly normalize inputs.
        return Err("Input must be in range [1, p-1]");
    }
    
    // a^(p-2) mod p
    // Constant-time note: This `mod_pow` is NOT constant time (branching on exp bits).
    // For security, use a "Montgomery Ladder" or constant-time exponentiation.
    Ok(mod_pow(a, p - 2, p))
}


pub fn entry_point() {
    // Scenario 1: Prime Modulus (e.g., p = 17)
    // We want inverse of 3 mod 17.
    // FLT approach: 3^(17-2) = 3^15 mod 17
    let p = 17;
    let a = 5;
    
    match inverse_fermat(a, p) {
        Ok(inv) => println!("FLT: Inverse of {} mod {} is {}", a, p, inv),
        Err(e) => println!("FLT Error: {}", e),
    }
}