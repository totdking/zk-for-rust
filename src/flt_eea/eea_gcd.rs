
/// Gets the coefficients `(x,y)` and the gcd `g` of the variables `a` and `b` in the equation `ax + by = g`
pub fn eea_gcd(a: i64, m: i64) -> (i64, i64, i64) {
    // coefficient of a
    let (mut t, mut new_t) = (0, 1);
    // remainder
    let (mut r, mut new_r) = (a, m);
    // coefficient of b
    let (mut s, mut new_s) = (1, 0);
    
    while new_r != 0 {
        let q = r / new_r;
        (new_r, r) = (r - q * new_r, new_r);
        (new_s, s) = (s - q * new_s, new_s);
        (new_t, t) = (t - q * new_t, new_t);
    }
    return (s, t, r);
}