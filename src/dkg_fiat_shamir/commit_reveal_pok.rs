//! Commit-Reveal Proof of Knowledge
//! 
//! Using schnorr to prove knowledge of a discrete log without revealing the private key

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// --- 1. MOCK CRYPTO PRIMITIVES ---
// In production, use `k256` or `curve25519-dalek`

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GroupElement(u64); // Simulates a Public Key (Point)

#[derive(Debug, Clone, Copy)]
struct Scalar(u64);       // Simulates a Private Key

impl GroupElement {
    // Simulates G * scalar
    fn generator_mul(s: Scalar) -> Self {
        GroupElement(s.0.wrapping_mul(5)) // Mock generator logic
    }
    
    // Simulates Point Addition
    fn add(self, other: Self) -> Self {
        GroupElement(self.0.wrapping_add(other.0))
    }
}

// --- 2. SCHNORR PROOF OF KNOWLEDGE (PoK) ---

#[derive(Debug, Clone)]
struct SchnorrProof {
    commitment: GroupElement, // R = k*G
    response: Scalar,         // s = k + e*x
}

impl SchnorrProof {
    // Prover: "I know x for P = xG"
    fn prove(secret_x: Scalar, public_p: GroupElement) -> Self {
        // 1. Generate random nonce k
        let k = Scalar(12345); // In prod, use strictly explicit RNG
        let r_commitment = GroupElement::generator_mul(k);
        
        // 2. Calculate Challenge e = Hash(P, R)
        // Fiat-Shamir Heuristic
        let challenge = Self::hash_challenge(public_p, r_commitment);
        
        // 3. Calculate Response s = k + (e * x)
        let s_val = k.0.wrapping_add(challenge.0.wrapping_mul(secret_x.0));
        
        SchnorrProof {
            commitment: r_commitment,
            response: Scalar(s_val),
        }
    }
    
    // Verifier: "Does sG == R + eP?"
    fn verify(&self, public_p: GroupElement) -> bool {
        let challenge = Self::hash_challenge(public_p, self.commitment);
        
        // LHS: s * G
        let lhs = GroupElement::generator_mul(self.response);
        
        // RHS: R + (e * P)
        let e_times_p = GroupElement(public_p.0.wrapping_mul(challenge.0));
        let rhs = self.commitment.add(e_times_p);
        
        lhs == rhs
    }
    
    fn hash_challenge(p: GroupElement, r: GroupElement) -> Scalar {
        let mut hasher = DefaultHasher::new();
        p.hash(&mut hasher);
        r.hash(&mut hasher);
        Scalar(hasher.finish() % 1000) // Small modulus for mock
    }
}

// --- 3. COMMIT-REVEAL SCHEME ---

struct Participant {
    id: String,
    secret: Scalar,
    public_key: GroupElement,
    salt: u64, // Blinding factor for the commit
}

impl Participant {
    fn new(id: &str, secret_val: u64) -> Self {
        let secret = Scalar(secret_val);
        Participant {
            id: id.to_string(),
            secret,
            public_key: GroupElement::generator_mul(secret),
            salt: 999, // Random salt in production
        }
    }
    
    // Phase 1: Publish Hash(Public Key || Salt)
    fn generate_commitment(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.public_key.hash(&mut hasher);
        self.salt.hash(&mut hasher);
        hasher.finish()
    }
    
    // Phase 2: Open the commitment
    fn reveal(&self) -> (GroupElement, u64, SchnorrProof) {
        let pok = SchnorrProof::prove(self.secret, self.public_key);
        (self.public_key, self.salt, pok)
    }
}

// --- 4. THE PROTOCOL EXECUTION ---

pub fn entry_point() {
    println!("--- DKG Security Simulation ---");

    // 1. Setup Participants
    let alice = Participant::new("Alice", 10);
    // Malory WANTS to do: public_key = M - Alice_PK
    // But he needs Alice_PK first.
    let malory = Participant::new("Malory", 666); 

    // --- PHASE 1: COMMITMENT ---
    println!("\n[Phase 1] Broadcasting Commitments...");
    let alice_commit = alice.generate_commitment();
    
    // ATTACK FOILED HERE:
    // Malory sees `alice_commit` (a hash). 
    // He cannot mathematically deduce Alice's Public Key from the hash.
    // He is forced to commit to his own key BLINDLY.
    let malory_commit = malory.generate_commitment(); 
    
    println!("Alice published hash: {:x}", alice_commit);
    println!("Malory published hash: {:x}", malory_commit);

    // --- PHASE 2: REVEAL ---
    println!("\n[Phase 2] Revealing Values...");
    let (alice_pk, alice_salt, alice_pok) = alice.reveal();
    let (malory_pk, malory_salt, malory_pok) = malory.reveal();

    // --- PHASE 3: VERIFICATION ---
    println!("\n[Phase 3] Verification...");

    // Verify Alice
    let mut hasher = DefaultHasher::new();
    alice_pk.hash(&mut hasher);
    alice_salt.hash(&mut hasher);
    let derived_hash = hasher.finish();
    
    if derived_hash == alice_commit && alice_pok.verify(alice_pk) {
        println!("Alice is VALID (Commitment matched + PoK verified)");
    } else {
        println!("Alice is INVALID");
    }

    // Verify Malory
    // Let's imagine Malory tries to swap his key NOW to cancel Alice out
    // He calculates: Fake_Key = Malory_Key - Alice_PK
    // But he committed to Malory_Key in Phase 1.
    
    // If he presents the Fake_Key now:
    let fake_key_attempt = GroupElement(malory_pk.0.wrapping_sub(alice_pk.0));
    
    let mut check_hasher = DefaultHasher::new();
    fake_key_attempt.hash(&mut check_hasher); // He tries to open the commitment with the fake key
    malory_salt.hash(&mut check_hasher);
    let check_hash = check_hasher.finish();
    
    if check_hash != malory_commit {
        println!("SECURITY ALERT: Malory's reveal does not match his commitment!");
        println!("Malory tried to change his key after seeing Alice's, but the Hash caught him.");
    }
}