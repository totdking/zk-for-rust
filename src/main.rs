mod nonce_reuse;
use nonce_reuse::*;

fn main() {
    // Real world example (simplified numbers)
    // The Sony PS3 hack used this exact logic because they set k = static number.
    let recovered_key = recover_private_key(10, 20, 45, 55, 5, 100);
    println!("CRITICAL FAILURE: Private Key is {}", recovered_key);
}