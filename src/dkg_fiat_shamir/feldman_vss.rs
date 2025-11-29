//! Feldman's Verifiable Secret Sharing
//! Uses C = g^a_0, g^a_1, ... as commitments
//! Verifies share y = P(i) = a_0 + a_1*i + a_2*i^2 + ... + a_n*i^n
//! 

// --- Mocking the Cryptographic Group (e.g., Elliptic Curve Point) ---
#[derive(Debug, Clone, Copy, PartialEq)]
struct GroupElement(u128); // "g^x"
#[derive(Debug, Clone, Copy)]
struct Scalar(u128);       // "x" (The private data)

// Mock Constants (In reality, these are massive numbers)
const MODULUS: u128 = 1009; // Order of the group

impl GroupElement {
    /// Simulating g^scalar
    fn generator_pow(s: Scalar) -> Self {
        // Just a mock modular exponentiation for logic demonstration
        // g = 5
        let mut base = 5u128;
        let mut exp = s.0;
        let mut res = 1u128;
        while exp > 0 {
            if exp % 2 == 1 { res = (res * base) % MODULUS; }
            base = (base * base) % MODULUS;
            exp /= 2;
        }
        GroupElement(res)
    }

    /// Group Operation (Point Addition corresponds to mult in modular groups)
    fn combine(&self, other: Self) -> Self {
        GroupElement((self.0 * other.0) % MODULUS)
    }
    
    /// Scalar Multiplication (g^a)^b = g^{ab}
    fn scalar_mul(&self, scalar: u128) -> Self {
        let mut base = self.0;
        let mut exp = scalar;
        let mut res = 1u128;
        while exp > 0 {
            if exp % 2 == 1 { res = (res * base) % MODULUS; }
            base = (base * base) % MODULUS;
            exp /= 2;
        }
        GroupElement(res)
    }
}

// --- The VSS Logic ---

/// Verifies a share against public commitments
/// 
/// * `index` (i): The x-coordinate of the node (Public)
/// * `share` (y): The private share value received (Private)
/// * `commitments` (C): The list of g^a_0, g^a_1... (Public)
fn verify_share(index: u128, share: Scalar, commitments: &[GroupElement]) -> bool {
    // 1. Calculate LHS: g^y
    let lhs = GroupElement::generator_pow(share);
    
    // 2. Calculate RHS: Product of (C_j)^(i^j)
    // Formula: C_0 * (C_1)^i * (C_2)^(i^2) ...
    let mut rhs = GroupElement(1); // Identity element
    
    for (j, commitment) in commitments.iter().enumerate() {
        // Calculate power: i^j
        let i_pow_j = index.pow(j as u32); 
        
        // Calculate term: (C_j)^(i^j)
        let term = commitment.scalar_mul(i_pow_j);
        
        // Combine into running product
        rhs = rhs.combine(term);
    }
    
    // 3. Check for equality
    lhs == rhs
}

pub fn entry_point() {
    println!("--- Feldman VSS Simulation ---");
    
    // Scenario: P(x) = 5 + 2x
    // Secret (a0) = 5
    // Slope (a1) = 2
    
    // 1. Dealer generates commitments (Publicly broadcast)
    let c0 = GroupElement::generator_pow(Scalar(5)); // g^5
    let c1 = GroupElement::generator_pow(Scalar(2)); // g^2
    let commitments = vec![c0, c1];
    
    println!("Commitments broadcasted: {:?}", commitments);
    
    // 2. Dealer sends Share to Node #3
    // x = 3
    // y = 5 + 2(3) = 11
    let node_index = 3;
    let node_share = Scalar(11);
    
    // 3. Node #3 verifies the share
    let is_valid = verify_share(node_index, node_share, &commitments);
    
    if is_valid {
        println!("SUCCESS: The share is valid against the commitments.");
    } else {
        println!("ALARM: The Dealer is lying!");
    }
    
    // 4. Test with a fake share (Tampering)
    let fake_share = Scalar(12); // Wrong value
    let is_fake_valid = verify_share(node_index, fake_share, &commitments);
     if !is_fake_valid {
        println!("SUCCESS: The system detected the fake share.");
    }
}