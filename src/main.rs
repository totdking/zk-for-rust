// mod dkg_fiat_shamir;
// use dkg_fiat_shamir::commit_reveal_pok;

mod flt_eea;
use flt_eea::eea_gcd;
fn main() {
    // let _ = ceaser_cipher("ITQZ FUXF BMPPXQ MFFUFGPQ");
    // //
    println!("eea gcd {:?}", eea_gcd::eea_gcd(7, 13))
}
