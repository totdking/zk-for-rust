use std::ops::{Add, Sub, Mul, Div, Rem};

// A small prime for demonstration (Finite Field GF(251))
// In production, this would be a U256 prime.
const PRIME: i128 = 251;

#[derive(Debug, Clone, Copy)]
struct FieldElement(i128);

impl FieldElement {
    fn new(num: i128) -> Self {
        // Handle negative modulo correctly
        let rem = num % PRIME;
        FieldElement(
            if rem < 0 { 
                rem + PRIME 
            } else 
            { 
                rem 
            }
        )
    }

    /// Extended Euclidean Algorithm for modular inverse
    fn inverse(&self) -> Self {
        // r is remainder
        // t is the coefficient
        // quotient is the quotient of the division of r by new_r
        let (mut t, mut new_t) = (0, 1);
        let (mut r, mut new_r) = (PRIME, self.0);

        while new_r != 0 {
            let quotient = r / new_r;
            t = t - quotient * new_t;
            std::mem::swap(&mut t, &mut new_t);
            r = r - quotient * new_r;
            std::mem::swap(&mut r, &mut new_r);
        }

        if r > 1 { panic!("{:?} is not invertible", self.0); }
        if t < 0 { t += PRIME; }
        FieldElement::new(t)
    }
}

// Operator Overloading for clean math syntax
impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self { FieldElement::new(self.0 + other.0) }
}
impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self { FieldElement::new(self.0 - other.0) }
}
impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self { FieldElement::new(self.0 * other.0) }
}
impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self { self * other.inverse() } // Div is mul by inverse
}

/// lagrange interpolation
/// 
/// uses **O(n^2)** time complexity
fn lagrange_interpolate(x_target: FieldElement, points: &[(FieldElement, FieldElement)]) -> FieldElement {
    let k = points.len();
    let mut result = FieldElement::new(0);

    for j in 0..k {
        let (x_j, y_j) = points[j];
        
        // Calculate Basis Polynomial L_j(x)
        let mut l_j = FieldElement::new(1);
        for i in 0..k {
            if i != j {
                let (x_i, _) = points[i];
                // L_j(x) = product of (x_target - x_i) / (x_j - x_i)
                let numerator = x_target - x_i;
                let denominator = x_j - x_i;
                l_j = l_j * (numerator / denominator);
            }
        }
        
        // P(x) = sum of y_j * L_j(x)
        result = result + (y_j * l_j);
    }
    result
}

pub fn entry_point() {
    // Scenario: We have 3 shares of a secret.
    // The polynomial is hidden, but let's assume P(x) = 12 + 4x + 3x^2 (mod 251)
    // Secret (P(0)) should be 12.
    
    // Shares: (x, y)
    // x=1 -> 12 + 4 + 3 = 19
    // x=2 -> 12 + 8 + 12 = 32
    // x=3 -> 12 + 12 + 27 = 51
    
    let shares = vec![
        (FieldElement::new(1), FieldElement::new(19)),
        (FieldElement::new(2), FieldElement::new(32)),
        (FieldElement::new(3), FieldElement::new(51)),
    ];

    println!("Attempting to recover secret (P(0))...");
    
    // We want to find y at x = 0
    let secret = lagrange_interpolate(FieldElement::new(0), &shares);
    
    println!("Recovered Secret: {:?}", secret);
    
    assert_eq!(secret.0, 12);
    println!("integrity check passed: Secret is 12 \n");
    // let ai_div = FieldElement::new(4);
    // let ai_inv = ai_div.inverse();
    // println!("the inverse of {:?} is {:?}", ai_div, ai_inv);

    // let own_div = ai_div.eea_inv();
    // println!("the inverse of {:?} is {:?}", ai_div, own_div);

}

impl FieldElement {
    fn eea_inv(&self) -> FieldElement{
        if self.0 == 0{
            panic!("{:?} has no multiplicative inverse", self.0);
        }
        if self.0 >= PRIME {
            panic!("{:?} is not a member of valid fields in the finite field: {:?} ", self.0, PRIME);
        }
        let (mut old_t, mut new_t) = (0, 1);
        let (mut old_r, mut new_r) = (PRIME, self.0);

        // while new_r != 0 {
        //     let q = old_r / new_r;
        //     old_t = new_t;
        //     new_t = old_t - q * new_t;
        //     old_r = new_r;
        //     new_r = old_r - q * new_r;
        // }
        while new_r !=0 {
            let q = old_r/ new_r;
            (old_t, new_t) = (new_t, old_t - q * new_t);
            (old_r, new_r) = (new_r, old_r - q * new_r);
        }

        if old_r > 1 {
            panic!("{:?} is not invertible", self.0);
        }
        if old_t < 0 {
            old_t += PRIME;
        }
        FieldElement::new(old_t)
    }
}