use std::ops::{Add, Div, Mul, Sub};

/// Define the field modulus P = 17 for this drill
/// Curve: y^2 = x^3 + 1
const P: i64 = 17;
const A: i64 = 0; // Coefficient a in y^2 = x^3 + ax + b
const B: i64 = 1; // Coefficient b

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldElement {
    pub value: i64,
}

impl FieldElement {
    pub fn new(value: i64) -> Self {
        let mut v = value % P;
        if v < 0 {
            v += P;
        }
        Self { value: v }
    }

    /// Inverse using fermat's little theorem
    pub fn inverse(&self) -> Self {
        if self.value == 0 {
            panic!("Cannot invert 0");
        }
        // Fermat's Little Theorem: a^(p-2) mod p
        // Inverse of a is a^(p-2) mod p
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
        Self::new(res)
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ECPoint {
    Infinity,
    Point { x: FieldElement, y: FieldElement },
}

impl ECPoint {
    pub fn new(x: i64, y: i64) -> Self {
        ECPoint::Point {
            x: FieldElement::new(x),
            y: FieldElement::new(y),
        }
    }

    pub fn add(self, other: Self) -> Self {
        match (self, other) {
            (ECPoint::Infinity, _) => other,
            (_, ECPoint::Infinity) => self,
            (ECPoint::Point { x: x1, y: y1 }, ECPoint::Point { x: x2, y: y2 }) => {
                // Case 1: x1 == x2
                if x1 == x2 {
                    // Case 1a: y1 != y2 (vertical line, e.g. P + (-P))
                    if y1 != y2 {
                        return ECPoint::Infinity;
                    }

                    // Case 1b: y1 == 0 (tangent is vertical)
                    if y1.value == 0 {
                        return ECPoint::Infinity;
                    }

                    // Case 1c: Point Doubling (P + P)
                    // Formula: s = (3x1^2 + a) / 2y1
                    // x3 = s^2 - 2x1
                    // y3 = s(x1 - x3) - y1

                    // TODO: Implement point doubling
                    // Hint: You need to calculate the slope 's' first.
                    // Remember to use FieldElement operations (+, -, *, /).
                    let s = (FieldElement::new(3) * x1 * x1 + FieldElement::new(A)) / (FieldElement::new(2) * y1);
                    let x3 = (s * s) - (FieldElement::new(2) * x1);
                    let y3 = s * (x1 - x3) - y1;

                    return ECPoint::Point { x: x3, y: y3 };
                } else {
                    // Case 2: Point Addition (P + Q where P != Q)
                    // Formula: s = (y2 - y1) / (x2 - x1)
                    // x3 = s^2 - x1 - x2
                    // y3 = s(x1 - x3) - y1

                    // TODO: Implement point addition
                    let s = (y2 - y1) / (x2 - x1);
                    let x3 = (s * s) - x1 - x2;
                    let y3 = s * (x1 - x3) - y1;
                    return ECPoint::Point { x: x3, y: y3 };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_addition_distinct() {
        // Curve: y^2 = x^3 + 1 mod 17
        // P1 = (0, 1)
        // P2 = (1, 6)
        // Expected: (7, 15)
        let p1 = ECPoint::new(0, 1);
        let p2 = ECPoint::new(1, 6);
        let expected = ECPoint::new(7, 15);

        let result = p1.add(p2);
        println!("(0,1) + (1,6) = {:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_point_doubling() {
        // P = (1, 6)
        // 2P = (14, 12)
        let p = ECPoint::new(1, 6);
        let expected = ECPoint::new(14, 12);

        let result = p.add(p);
        println!("2 * (1,6) = {:?}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_identity() {
        let p = ECPoint::new(1, 6);
        assert_eq!(p.add(ECPoint::Infinity), p);
        assert_eq!(ECPoint::Infinity.add(p), p);
    }

    #[test]
    fn test_inverse() {
        // P = (0, 1)
        // -P = (0, -1) = (0, 16)
        // P + (-P) = Infinity
        let p1 = ECPoint::new(0, 1);
        let p2 = ECPoint::new(0, 16);
        assert_eq!(p1.add(p2), ECPoint::Infinity);
    }
}
