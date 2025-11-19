mod monoid;
pub use monoid::*;

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