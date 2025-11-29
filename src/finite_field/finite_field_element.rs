use std::ops::{Add, Sub, Mul};
//P = 17
const P: i32 = 17;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FieldElement{
    value: i32
}

impl FieldElement{
    /// Constructor ensuring we stay inside the field [0, P-1]
    pub fn new(val: i32) -> Self {
        let mut res = val % P;
        if res < 0 {
            res += P;
        }
        FieldElement { value: res }
    }

    /// Find Inverse: a^(P-2) mod P (Fermat's Little Theorem)
    /// Used for division.
    pub fn inverse(&self) -> Self {
        let mut base = self.value;
        let mut exp = P - 2;
        let mut res = 1;
        
        while exp > 0 {
            if exp % 2 == 1 { 
                res = (res * base) % P; 
            }
            base = (base * base) % P;
            exp /= 2;
        }
        FieldElement::new(res)
    }
}

/// Rule 1: Closure (The result is always a FieldElement)
impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        FieldElement::new(self.value + other.value)
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        FieldElement::new(self.value - other.value)
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        FieldElement::new(self.value * other.value)
    }
}

fn main() {
    let a = FieldElement::new(15);
    let b = FieldElement::new(4);

    // 1. Addition (Group Operation)
    // 15 + 4 = 19. 19 mod 17 = 2.
    let sum = a + b;
    println!("Sum: {:?}", sum); // Output: FieldElement { value: 2 }

    // 2. Associativity Check
    let c = FieldElement::new(3);
    assert_eq!((a + b) + c, a + (b + c)); 
    println!("Associativity holds.");

    // 3. Identity Check (0)
    let zero = FieldElement::new(0);
    assert_eq!(a + zero, a);
    println!("Identity holds.");

    // 4. Inverse Check
    // If we want a / b, we do a * inverse(b)
    // This is critical for ZK polynomial evaluations
    let inv_b = b.inverse();
    println!("Inverse of 4 mod 17 is: {:?}", inv_b); // Should be 13 (because 4*13 = 52, 52 mod 17 = 1)
}