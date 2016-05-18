extern crate gmp;

use types::test_type;

#[test]
fn test_mpf_params() {
    test_type("NUMERIC", &[
              (Some(mpf::Mpf::new(32).set_from_str("10.5", 10)), ""),
              (None, "NULL")])
}
