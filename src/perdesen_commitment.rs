use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{PrimeField, UniformRand};
use ark_bn254::{G1Projective as Point, Fr as Scalar}; // Using BN254 Curve
use rand::{Rng, rng};

struct Pedersen {
    g: Point,
    h: Point,
}

impl Pedersen {
    // 1. Setup: Generate the generator points
    fn setup() -> Self {
        let mut rng = rng();
        // In production, H must be derived verifiably (e.g., MapToCurve hash)
        // to ensure no one knows the discrete log relationship.
        let g = Point::prime_subgroup_generator();
        let h = Point::rand(&mut rng); 
        
        Self { g, h }
    }

    // 2. Commit: C = v*G + r*H
    // We use 'Scalar' (Fr) for the value because that's what we count with.
    fn commit(&self, value: Scalar, blinding: Scalar) -> Point {
        // mul performs scalar multiplication on the curve
        // + adds two curve points together
        (self.g.mul(value.into_repr())) + (self.h.mul(blinding.into_repr()))
    }
}

fn main() {
    let scheme = Pedersen::setup();
    let mut rng = thread_rng();

    // --- User A: Commits to Value 50 ---
    let value_a = Scalar::from(50u64);
    let blind_a = Scalar::rand(&mut rng); // Random secret to hide the 50
    let comm_a = scheme.commit(value_a, blind_a);

    // --- User B: Commits to Value 25 ---
    let value_b = Scalar::from(25u64);
    let blind_b = Scalar::rand(&mut rng);
    let comm_b = scheme.commit(value_b, blind_b);

    // --- The Magic: "Homomorphic Addition" ---
    // We add the COMMITMENTS (Curve Points) together.
    // We do NOT touch the secret values.
    let comm_sum = comm_a + comm_b;

    // --- Verification ---
    // Later, we want to prove that comm_sum really is a commitment to 75 (50+25).
    // We simply add our secret values and blindings locally...
    let expected_value = value_a + value_b; // 75
    let expected_blind = blind_a + blind_b;
    
    // ...and regenerate the commitment.
    let verification_comm = scheme.commit(expected_value, expected_blind);

    // If the points match, the math works!
    assert_eq!(comm_sum, verification_comm);
    
    println!("Success: The sum of the commitments equals the commitment of the sum.");
}