mod dkg_fiat_shamir;
use dkg_fiat_shamir::lagrange_interpolation;

fn main() {
    println!("Run 'cargo test' to test your EC implementation!");
    lagrange_interpolation::entry_point();
}
