// Modular Square Root Algorithms
// These algorithms find x such that x^2 ≡ a (mod p)

/// Simple modular exponentiation: base^exp mod m
/// Time Complexity: O(log exp)
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

/// Algorithm 1: Direct formula for primes p ≡ 3 (mod 4)
/// Time Complexity: O(log p) - FASTEST when applicable
///
/// For primes where p ≡ 3 (mod 4), the square root can be computed directly:
/// r = a^((p+1)/4) mod p
///
/// Why this works:
/// - If p ≡ 3 (mod 4), then (p+1)/4 is an integer
/// - r^2 = a^((p+1)/2) = a^((p-1)/2) * a
/// - By Euler's criterion, a^((p-1)/2) ≡ 1 (mod p) for quadratic residues
/// - Therefore r^2 ≡ a (mod p)
pub fn sqrt_mod_p3mod4(a: u128, p: u128) -> Option<u128> {
    // Check if p ≡ 3 (mod 4)
    if p % 4 != 3 {
        return None;
    }

    // Check if a is a quadratic residue using Euler's criterion
    // a^((p-1)/2) should equal 1 for quadratic residues
    let euler = mod_pow(a % p, (p - 1) / 2, p);
    if euler != 1 {
        return None; // Not a quadratic residue
    }

    // Compute r = a^((p+1)/4) mod p
    let r = mod_pow(a % p, (p + 1) / 4, p);
    Some(r)
}

/// Algorithm 2: Tonelli-Shanks Algorithm
/// Time Complexity: O(log^2 p) - Works for ALL primes
///
/// This is the most general algorithm that works for any prime p.
/// It's more complex but handles all cases including p ≡ 1 (mod 4)
pub fn tonelli_shanks(n: u128, p: u128) -> Option<u128> {
    // Check if n is a quadratic residue
    let euler = mod_pow(n % p, (p - 1) / 2, p);
    if euler != 1 {
        return None; // Not a quadratic residue
    }

    // Special case: p ≡ 3 (mod 4) - use faster method
    if p % 4 == 3 {
        return Some(mod_pow(n % p, (p + 1) / 4, p));
    }

    // Write p - 1 = Q * 2^S where Q is odd
    let mut q = p - 1;
    let mut s = 0u128;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    // Find a quadratic non-residue z
    let mut z = 2u128;
    while mod_pow(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }

    // Initialize variables
    let mut m = s;
    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(n, q, p);
    let mut r = mod_pow(n, (q + 1) / 2, p);

    // Main loop
    loop {
        if t == 0 {
            return Some(0);
        }
        if t == 1 {
            return Some(r);
        }

        // Find the least i such that t^(2^i) = 1
        let mut i = 1u128;
        let mut temp = (t * t) % p;
        while temp != 1 && i < m {
            temp = (temp * temp) % p;
            i += 1;
        }

        // Update values
        let b = mod_pow(c, 1 << (m - i - 1), p);
        m = i;
        c = (b * b) % p;
        t = (t * c) % p;
        r = (r * b) % p;
    }
}

/// Algorithm 3: Cipolla's Algorithm
/// Time Complexity: O(log^2 p) - Alternative to Tonelli-Shanks
///
/// This algorithm uses arithmetic in a quadratic extension field.
/// It's often faster in practice than Tonelli-Shanks.
pub fn cipolla(n: u128, p: u128) -> Option<u128> {
    let n = n % p;

    // Check if n is a quadratic residue
    let euler = mod_pow(n, (p - 1) / 2, p);
    if euler != 1 {
        return None;
    }

    // Special case
    if n == 0 {
        return Some(0);
    }

    // Find a such that a^2 - n is a quadratic non-residue
    let mut a = 0u128;
    let mut omega2;
    loop {
        omega2 = (a * a + p - n) % p;
        if mod_pow(omega2, (p - 1) / 2, p) == p - 1 {
            break;
        }
        a += 1;
    }

    // Compute (a + ω)^((p+1)/2) in the extension field
    // where ω^2 = a^2 - n
    let (r, _) = field_pow(a, 1, (p + 1) / 2, omega2, p);
    Some(r)
}

/// Helper function for Cipolla: exponentiation in quadratic extension field
/// Computes (x + y*ω)^exp where ω^2 = omega2
fn field_pow(x: u128, y: u128, mut exp: u128, omega2: u128, p: u128) -> (u128, u128) {
    let mut result_x = 1u128;
    let mut result_y = 0u128;
    let mut base_x = x;
    let mut base_y = y;

    while exp > 0 {
        if exp % 2 == 1 {
            // Multiply result by base
            let new_x = (result_x * base_x + result_y * base_y % p * omega2) % p;
            let new_y = (result_x * base_y + result_y * base_x) % p;
            result_x = new_x;
            result_y = new_y;
        }

        // Square base
        let new_x = (base_x * base_x + base_y * base_y % p * omega2) % p;
        let new_y = (2 * base_x * base_y) % p;
        base_x = new_x;
        base_y = new_y;

        exp >>= 1;
    }

    (result_x, result_y)
}
pub fn entrypoint() {
    let square = 18;
    let p = 47; // Prime where p ≡ 3 (mod 4)

    println!("Finding square root of {} mod {}\n", square, p);

    // Method 1: Brute force (slow - O(p))
    println!("1. Brute Force O(p): {}", get_root(square, p));

    // Method 2: Direct formula for p ≡ 3 (mod 4) - FASTEST
    if let Some(r) = sqrt_mod_p3mod4(square, p) {
        println!("2. p≡3(mod 4) O(log p): {}", r);
    }

    // Method 3: Tonelli-Shanks (works for all primes)
    if let Some(r) = tonelli_shanks(square, p) {
        println!("3. Tonelli-Shanks O(log²p): {}", r);
    }

    // Method 4: Cipolla's algorithm
    if let Some(r) = cipolla(square, p) {
        println!("4. Cipolla O(log²p): {}", r);
    }

    // Test with a larger prime to show performance difference
    println!("\n--- Testing with larger prime ---");
    let large_p = 1000000007u128; // Large prime ≡ 3 (mod 4)
    let large_square = 123456;

    println!("Finding square root of {} mod {}", large_square, large_p);

    // Only use fast algorithms for large primes!
    if let Some(r) = sqrt_mod_p3mod4(large_square, large_p) {
        println!("p≡3(mod 4) method: {}", r);
        println!(
            "Verification: {}² mod {} = {}",
            r,
            large_p,
            (r * r) % large_p
        );
    }
}

fn get_root(square: u128, p: u128) -> u128 {
    for i in 0..p {
        if i * i % p == square {
            return i;
        }
    }
    return p;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_mod_p3mod4() {
        // Test with p = 47 (which is 3 mod 4)
        let result = sqrt_mod_p3mod4(18, 47);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!((r * r) % 47, 18);
        println!("sqrt(18) mod 47 = {}", r);
    }

    #[test]
    fn test_tonelli_shanks() {
        // Test with p = 47
        let result = tonelli_shanks(18, 47);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!((r * r) % 47, 18);
        println!("Tonelli-Shanks: sqrt(18) mod 47 = {}", r);

        // Test with a larger prime p = 1009 (which is 1 mod 4)
        let result2 = tonelli_shanks(56, 1009);
        assert!(result2.is_some());
        let r2 = result2.unwrap();
        assert_eq!((r2 * r2) % 1009, 56);
        println!("Tonelli-Shanks: sqrt(56) mod 1009 = {}", r2);
    }

    #[test]
    fn test_cipolla() {
        // Test with p = 47
        let result = cipolla(18, 47);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!((r * r) % 47, 18);
        println!("Cipolla: sqrt(18) mod 47 = {}", r);
    }

    #[test]
    fn test_non_residue() {
        // 5 is not a quadratic residue mod 47
        assert!(sqrt_mod_p3mod4(5, 47).is_none());
        assert!(tonelli_shanks(5, 47).is_none());
        assert!(cipolla(5, 47).is_none());
    }

    #[test]
    fn compare_algorithms() {
        let test_cases = vec![(18, 47), (10, 47), (2, 47)];

        for (a, p) in test_cases {
            let r1 = sqrt_mod_p3mod4(a, p);
            let r2 = tonelli_shanks(a, p);
            let r3 = cipolla(a, p);

            println!("\nsqrt({}) mod {}:", a, p);
            if let Some(x) = r1 {
                println!("  p≡3(mod 4) method: {}", x);
            }
            if let Some(x) = r2 {
                println!("  Tonelli-Shanks: {}", x);
            }
            if let Some(x) = r3 {
                println!("  Cipolla: {}", x);
            }

            // All methods should agree (or all return None)
            assert_eq!(r1.is_some(), r2.is_some());
            assert_eq!(r2.is_some(), r3.is_some());
        }
    }
}
