extern crate gmp;

use types::test_type;
use self::gmp::mpf;

#[test]
fn test_mpf_params() {
    let mut i = mpf::Mpf::zero();
    i.set_from_str("101.6", 10);
    test_type("NUMERIC", &[
              (Some(i), "-1.11111"),
              (None, "NULL")])
}
