// mod dkg_fiat_shamir;
// use dkg_fiat_shamir::commit_reveal_pok;

mod ceaser_cipher;
mod flt_eea;
mod modular_sqrt;

use flt_eea::eea_gcd;
use modular_sqrt::{cipolla, sqrt_mod_p3mod4, tonelli_shanks};
fn main() {
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
