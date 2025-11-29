//! This is for finding the multiplicative inverse in finite fields 
//! making use of the Extended Euclidean algorithm
//! 


pub mod finite_field_mul_inv;
use finite_field_mul_inv::*;
fn main(){
    let mul = 8;
    let p = 11;
    let res = eea_inv(mul, p);
    println!("res is {:?}", res);
}


/// The correct one ai did
/// 
/// Finite field P should always be a prime 
/// 
/// This was much better and panicked if the number did not have an inverse in the finite field
/// 
/// Unlike the other that just kept looping on overflow
pub fn eea_inv(num: u128, p: u128) -> u128{
    // Add a constraint to prevent non prime numbers to be used as P
    if num >= p {
        panic!("The number: {:?} is not a member of valid fields in the finite field: {:?} ", num, p);
    }
    if num == 0 {
        panic!("0 has no multiplicative inverse in any field");
    }

    // Use i64 to handle potential -ve numbers during calculation
    let (mut t , mut new_t) = (0, 1);
    let (mut r, mut new_r) = (p, num);

    // loop runs untli the remainder (new_r) is 0
    while new_r != 0 {
        let quotient = r / new_r;

        // Update the coefficients (t) and remainder (r)
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    // If the final remainder `r` (which is the gcd) is greater than 1
    // then num and p are not co-prime and the inverse doesnt exist
    if r > 1 {
        panic!("{:?} is not invertible", num);
    }
    if t < 0 {
        t = t + (p as i128);
    }

    // The resulting coefficient `t` can be negative.
    // We bring it into the valid range [0, p-1] by adding p.
    t as u128
}

/// Only allow primes to be used as the Finite field p
fn only_prime(p: u128) -> bool {
    if p <= 1 {
        return false;
    }
    if p <= 3 {
        return true;
    }
    if p % 2 == 0 || p % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= p {
        if p % i == 0 || p % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

/// Finding the multiplicative inverse of a number in finite field **P**
pub fn eea_inv2(num: u32, p: u32) -> u32{
    if num > p - 1 {
        panic!("The number: {:?} is not a member of valid fields in the finite field: {:?} ", num, p);
    }

    if num % p == 0 {
        panic!("{:?} has no multiplicative inverse in the finite field: {:?} ", num, p);
    }

    let mut target = 0;
    loop {
        let mul = num * target ;
        if mul % p == 1 {
            break;
        }
        target +=1;
    }
    println!("the multiplicative inverse of {:?} is {:?}", num, target);
    return target;
}