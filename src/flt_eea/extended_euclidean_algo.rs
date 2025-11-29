//! Standard "BigInt" style arithmetic for demonstration.
//! In production, use `crypto-bigint` or `num-bigint`.

/// 2. Extended Euclidean Algorithm Inverse
/// WORKS FOR: Any `m` where gcd(a, m) == 1 : Meaning, they must not be factors, multiples of each other
/// 
/// The greatest common divisor of the 2 numbers must be 1 for the inverse to exist
fn inverse_eea(a: i128, m: i128) -> Result<i128, &'static str> {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (m, a);

    while new_r != 0 {
        let quotient = r / new_r;
        
        // Parallel assignment to update (t, new_t) and (r, new_r)
        let temp_t = t - quotient * new_t;
        t = new_t;
        new_t = temp_t;
        
        let temp_r = r - quotient * new_r;
        r = new_r;
        new_r = temp_r;
    }

    if r > 1 {
        return Err("a is not invertible modulo m (GCD != 1)");
    }
    
    // Handle negative result from the subtraction
    if t < 0 {
        t = t + m;
    }

    Ok(t)
}

pub fn entry_point() {
    // Scenario 2: Composite Modulus (e.g., m = 20)
    // We want inverse of 3 mod 20.
    // FLT would FAIL here because 20 is not prime.
    // EEA works because gcd(3, 20) = 1.
    let m = 20;
    let b = 3;
    
    match inverse_eea(b, m) {
        Ok(inv) => println!("EEA: Inverse of {} mod {} is {}", b, m, inv),
        Err(e) => println!("EEA Error: {}", e),
    }
    
    // Verification
    // 3 * 7 = 21 == 1 mod 20. Correct.
}