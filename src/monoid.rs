use std::ops::Mul;

// --- 1. The Setup (The Monoids) ---

// The Plaintext Monoid (Salaries)
// Identity: 0
// Operation: Addition (+)

// The Ciphertext Monoid (Encrypted Data)
// Identity: 1 (because the underlying math uses multiplication)
// Operation: Multiplication (*)

#[derive(Debug, Clone, Copy)]
pub struct Ciphertext(pub u64); // Simplified for demo

// --- 2. The "Toy" Crypto System ---
// (WARNING: DO NOT USE IN PRODUCTION. This is a toy Paillier-like simulation)
// Real Paillier uses massive BigInts. We are simulating the property g^m * r^n mod n^2
pub struct ToyHomomorphicSystem {
    pub public_key: u64, // In this toy, just a shifting factor
}

impl ToyHomomorphicSystem {
    pub fn new() -> Self {
        Self { public_key: 10 } // Simplified base
    }

    // Encrypt: g^m (Simplified to public_key^m)
    pub fn encrypt(&self, m: u32) -> Ciphertext {
        // In reality, this includes random noise for security.
        // Here, we just show the algebraic property: Base^exponent
        Ciphertext(self.public_key.pow(m))
    }

    // Decrypt: log_base(c)
    pub fn decrypt(&self, c: Ciphertext) -> u32 {
        // In reality, this requires the private key (trapdoor).
        // Here we just reverse the math.
        (c.0 as f64).log(self.public_key as f64).round() as u32
    }
}

// --- 3. The Server Logic (The Monoid Implementation) ---

// Implementing the Monoid Operation for Ciphertext
// Remember: To ADD plaintexts, we must MULTIPLY ciphertexts.
impl Mul for Ciphertext {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Ciphertext(self.0 * rhs.0)
    }
}


fn main() {
    let sys = ToyHomomorphicSystem::new();

    // --- Step 1: Client Encrypts Data ---
    let salary_alice = 5; // $50k
    let salary_bob = 3;   // $30k
    
    let enc_alice = sys.encrypt(salary_alice);
    let enc_bob = sys.encrypt(salary_bob);

    println!("Server sees encrypted blobs: {:?} and {:?}", enc_alice, enc_bob);

    // --- Step 2: The Untrusted Server "Sums" the Data ---
    // The server DOES NOT know 5 or 3. It only sees the Ciphertexts.
    // It uses the Monoid property (Multiplication) to combine them.
    
    let enc_total = server_compute_sum(vec![enc_alice, enc_bob]);

    // --- Step 3: Client Decrypts the Result ---
    let decrypted_total = sys.decrypt(enc_total);

    println!("Client decrypts result: {}", decrypted_total);
    
    // Verification
    assert_eq!(decrypted_total, salary_alice + salary_bob); 
    println!("Success: 5 + 3 = {}", decrypted_total);
}

// The Server Function
// Notice: No Private Key needed! Just Monoid operations.
fn server_compute_sum(encrypted_values: Vec<Ciphertext>) -> Ciphertext {
    // Fold is the standard way to reduce a Monoid stream
    // Start with Identity (1 for multiplication), combine via Mul
    encrypted_values.into_iter().fold(Ciphertext(1), |acc, x| acc * x)
}