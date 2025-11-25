use std::ops::{Add, Mul, Sub, Div};

/// Field modulus P = 211 (a prime number)
const P: i128 = 211;
const A: i128 = 0;
const B: i128 = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldElement {
    pub value: i128,
}

impl FieldElement {
    pub fn new(value: i128) -> Self {
        let mut v = value % P;
        if v < 0 {
            v += P;
        }
        Self { value: v }
    }

    pub fn inverse(&self) -> Self {
        if self.value == 0 {
            panic!("Cannot invert 0");
        }
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
    
    /// Helper function for Projective field if the change in y or x is zero
    pub fn is_zero(&self) -> bool {
        self.value == 0
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
    fn div(self, other:Self) -> Self{
        self * other.inverse()
    }
}

/// Affine Point (x, y)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AffinePoint {
    Infinity,
    Point { x: FieldElement, y: FieldElement },
}

/// Projective Point (X, Y, Z)
/// Represents (X/Z, Y/Z) in Affine coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectivePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
}

impl ProjectivePoint {
    pub fn infinity() -> Self {
        Self {
            x: FieldElement::new(0),
            y: FieldElement::new(1),
            z: FieldElement::new(0),
        }
    }

    pub fn is_infinity(&self) -> bool {
        self.z.value == 0
    }
}

/// Convert from Affine to Projective
/// Formula: (x, y) -> (x, y, 1)
/// Infinity -> (0, 1, 0)
impl From<AffinePoint> for ProjectivePoint {
    fn from(p: AffinePoint) -> Self {
        // TODO: Implement conversion from Affine to Projective
        match p {
            AffinePoint::Infinity => Self::infinity(),
            AffinePoint::Point { x, y } => {
                Self{ x, y, z: FieldElement::new(1)}
            }
        }
    }
}

/// Convert from Projective to Affine
/// Formula: (X, Y, Z) -> (X/Z, Y/Z)
/// If Z == 0, return Infinity
impl From<ProjectivePoint> for AffinePoint {
    fn from(p: ProjectivePoint) -> Self {
        // TODO: Implement conversion from Projective to Affine
        // Hint: You need to find the inverse of Z
        let z_inv = p.z.inverse();
        let x = p.x * z_inv;
        let y = p.y * z_inv;
        AffinePoint::Point { x, y }
    }
}


impl ProjectivePoint {
    /// Point Doubling: 2P
    /// Formula for y^2 = x^3 + ax + b (where a=0 for this curve):
    /// if P is infinity, return infinity
    /// w = 3 * x^2
    /// s = y * z
    /// b = x * y * s
    /// h = w^2 - 8 * b
    /// x' = 2 * h * s
    /// y' = w * (4 * b - h) - 8 * y^2 * s^2
    /// z' = 8 * s^3
    /// Note: This is just one version of the formula. You can use any valid projective doubling formula for y^2 = x^3 + b.
    /// Simplified formula for a=0:
    /// W = 3 * X^2
    /// S = Y * Z
    /// B = X * Y * S
    /// H = W^2 - 8 * B
    /// X3 = 2 * H * S
    /// Y3 = W * (4 * B - H) - 8 * Y^2 * S^2
    /// Z3 = 8 * S^3
    pub fn double(&self) -> Self {
        // TODO: Implement point doubling in projective coordinates
        if self.is_infinity(){
            return Self::infinity();
        }
        let w = (FieldElement::new(3) * self.x * self.x) + FieldElement::new(A);
        let s = self.y * self.z;
        let b = self.x * self.y * s;
        let h = (w * w) - (FieldElement::new(8) * b);
        let x3 = FieldElement::new(2) * h * s;
        let y3 = w * ((FieldElement::new(4) * b) - h) - (FieldElement::new(8) * self.y * self.y * s * s);
        let z3 = FieldElement::new(8) * s * s * s;
        ProjectivePoint { x: x3, y: y3, z: z3 }
    }

    /// Point Addition: P + Q
    /// If P is infinity, return Q
    /// If Q is infinity, return P
    /// If P == Q, return double(P)
    /// Otherwise use the addition formula for projective coordinates.
    pub fn add(&self, other: &Self) -> Self {
        // TODO: Implement point addition in projective coordinates
        if self.is_infinity() {
            return *other;
        };
        if other.is_infinity() {
            return *self;
        };
        if self == other{
            return self.double();
        };
        // change in y
        let u = (other.y * self.z) - (self.y * other.z);
        // change in x
        let v = (other.x * self.z) - (self.x * other.z);
        if v.is_zero() {
            if u.is_zero() {
                return self.double();
            } else {
                // meaning it will have a vertical slope / tangent
                // k/0 to infinity
                return Self::infinity();
            }
        }
        let v2 = v * v;
        let v3 = v2 * v;
        let z1z2 = self.z * other.z;
        let a = (u * u * z1z2) - v3 - (FieldElement::new(2) * v2 * self.x * other.z);

        let x3 = v * a ;
        let y3 = u * ((v2 * self.x * other.z) - a) - (v3 * self.y * other.z);
        let z3 = v3 * z1z2;
        ProjectivePoint { x: x3, y: y3, z: z3 }
    }

    /// Scalar Multiplication: n * P
    /// Use the Double-and-Add algorithm
    pub fn scalar_mul(&self, scalar: u64) -> Self {
        // TODO: Implement scalar multiplication using Double-and-Add
        // unimplemented!("Implement ProjectivePoint::scalar_mul")
        if self.is_infinity() || scalar == 0 {
            return Self::infinity();
        }
        let mut result = Self::infinity();
        let mut point = *self;
        let mut scalar = scalar;
        while scalar > 0 {
            if scalar % 2 == 1 {
                result = result.add(&point);
            }
            point = point.double();
            scalar /= 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affine_to_projective() {
        let affine = AffinePoint::Point {
            x: FieldElement::new(10),
            y: FieldElement::new(20),
        };
        let proj: ProjectivePoint = affine.into();
        assert_eq!(proj.x.value, 10);
        assert_eq!(proj.y.value, 20);
        assert_eq!(proj.z.value, 1);

        let inf = AffinePoint::Infinity;
        let proj_inf: ProjectivePoint = inf.into();
        assert!(proj_inf.is_infinity());
    }

    #[test]
    fn test_projective_to_affine() {
        let proj = ProjectivePoint {
            x: FieldElement::new(20),
            y: FieldElement::new(40),
            z: FieldElement::new(2),
        };
        let affine: AffinePoint = proj.into();
        if let AffinePoint::Point { x, y } = affine {
            assert_eq!(x.value, 10); // 20 / 2
            assert_eq!(y.value, 20); // 40 / 2
        } else {
            panic!("Expected Point, got Infinity");
        }
    }

    #[test]
    fn test_projective_double() {
        // P = (3, 10) on y^2 = x^3 + 1 mod 211 (27 + 1 = 28, 100 != 28... wait, let's find a valid point)
        // x=4, y^2 = 64+1 = 65.
        // x=5, y^2 = 125+1 = 126.
        // Let's use a simple valid point if possible or just test logic consistency if we don't strictly enforce curve equation in types.
        // For this drill, we assume inputs are on curve.

        // Let's use the generator G=(0,1) for y^2 = x^3 + 1
        // 1 = 0 + 1. Correct.
        let g_affine = AffinePoint::Point {
            x: FieldElement::new(0),
            y: FieldElement::new(1),
        };
        let g: ProjectivePoint = g_affine.into();

        let g2 = g.double();
        let g2_affine: AffinePoint = g2.into();

        // Expected 2G:
        // s = (3x^2)/2y = 0/2 = 0
        // x3 = s^2 - 2x = 0
        // y3 = s(x - x3) - y = 0 - 1 = -1 = 210
        // 2G = (0, 210)

        if let AffinePoint::Point { x, y } = g2_affine {
            assert_eq!(x.value, 0);
            assert_eq!(y.value, 210);
        } else {
            panic!("Expected Point");
        }
    }

    #[test]
    fn test_scalar_mul() {
        // G = (0, 1)
        // 2G = (0, 210)
        // 3G = G + 2G
        // s = (210 - 1) / (0 - 0) -> Vertical line! 3G = Infinity.
        // Order is 3?
        // Let's check: 3 * (0,1)

        let g_affine = AffinePoint::Point {
            x: FieldElement::new(0),
            y: FieldElement::new(1),
        };
        let g: ProjectivePoint = g_affine.into();

        let g3 = g.scalar_mul(3);
        assert!(g3.is_infinity());

        let g2 = g.scalar_mul(2);
        let g2_affine: AffinePoint = g2.into();
        if let AffinePoint::Point { x, y } = g2_affine {
            assert_eq!(x.value, 0);
            assert_eq!(y.value, 210);
        } else {
            panic!("Expected Point");
        }
    }
}
