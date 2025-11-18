
// Vuln code 1
// /// Cracking discrete Log
// /// 
// /// IN a cyclic group where the finite field p is small, 
// /// 1. Base: Generator
// /// 2. Target: Public key
// /// 3. P: Finite field P
// pub fn crack_discrete_log(base: u32, target: u32, p: u32) -> Option<u32> {
//     let mut current = 1;
//     // Brute force: Try every possible exponent x from 0 to P-1
//     for x in 0..p {
//         if current == target {
//             return Some(x); // WE GOT EM. This is the private key.
//         }
//         // Move to next step: current = (current * base) % p
//         current = (current * base) % p;
//     }
//     None
// }
// // fn main() {
// //     let p = 17;
// //     let generator = 3;
// //     let public_key = 13; // This is 3^x mod 17. What is x?
// //     println!("--- 1. ATTACKING THE TRAPDOOR ---");
// //     match crack_discrete_log(generator, public_key, p) {
// //         Some(private_key) => println!("CRACKED: The private key is {}", private_key),
// //         None => println!("Failed to crack."),
// //     }
// // }

// // Vuln Code 2
// /// A VULNERABLE implementation
// #[derive(Debug, PartialEq, Clone, Copy)] // PartialEq checks exact bits, not math meaning
// pub struct BadFieldElement {
//     value: u32, // Using u32, allowing values > 17
// }
// impl BadFieldElement {
//     /// VULNERABILITY: No modulo check! 
//     /// It accepts 19, 36, 53... all as "valid" inputs.
//     pub fn new_unchecked(val: u32) -> Self {
//         BadFieldElement { value: val }
//     }
// }
// fn main() {
//     let p = 17;
//     println!("\n--- 2. EXPLOITING ALIASING (NON-CANONICAL INPUTS) ---"); 
//     // 1. Honest User Interaction
//     let user_id = BadFieldElement::new_unchecked(2);
//     let mut nullifier_set = vec![user_id]; // Store who has already acted
//     println!("User {:?} has voted.", user_id);
//     // 2. The Attacker tries to vote again using an Alias
//     // 19 is mathematically equal to 2 mod 17. 
//     // In a ZK circuit, the constraints would pass (19 - 2 = 17 = 0 mod 17).
//     let attacker_alias = BadFieldElement::new_unchecked(19);
//     // nullifier_set.push(attacker_alias);
//     // 3. The Flawed Check
//     if nullifier_set.contains(&attacker_alias) {
//         println!("ACCESS DENIED: You already voted.");
//     } else {
//         // The system thinks 19 is a different person than 2.
//         println!("ACCESS GRANTED: Voted again! (Double Spend Successful)");
//         println!("System sees: {:?} != {:?}", user_id, attacker_alias);
//     }
// }


// A secure fix
#[derive(Debug, PartialEq)]
pub struct SecureFieldElement {
    value: u32,
}

impl SecureFieldElement {
    // SECURE: Force the value into [0, P-1] immediately
    pub fn new(val: u32) -> Self {
        SecureFieldElement { value: val % 17 }
    }
}

// In the main function...
// let s1 = SecureFieldElement::new(2);
// let s2 = SecureFieldElement::new(19);
// s1 == s2 is now TRUE. The attack fails.


