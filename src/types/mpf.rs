extern crate gmp;

use std::io::prelude::*;

use self::gmp::mpf;
use types::{FromSql, ToSql, Type, IsNull, SessionInfo};
use Result;

impl FromSql for mpf::Mpf {
    fn from_sql<R: Read>(_: &Type, raw: &mut R, _: &SessionInfo) -> Result<mpf::Mpf> {
        let mut bytes = [0; 4];
        try!(raw.read_exact(&mut bytes));
        // println!("{:?}", bytes);
        let mut i = mpf::Mpf::zero();
        // let mut buffer = String::new();
        let mut new_bytes = Vec::new();
        raw.read_to_end(&mut new_bytes);
        // println!("{:?}", new_bytes);
        // raw.read_to_string(&mut buffer).unwrap();
        let s = String::from_utf8(new_bytes).unwrap();
        i.set_from_str(&s, 10);
        Ok(i)
    }

    accepts!(Type::Numeric);
}

impl ToSql for mpf::Mpf {
    fn to_sql<W: Write + ?Sized>(&self, _: &Type, out: &mut W, _: &SessionInfo) -> Result<IsNull> {
        // let mut cp = self.clone();
        // let mut e: c_long = 0;
        // let chars = {
        //     let eb = &mut e;
        //     cp.get_str(0, 10, eb)
        // };
        // let fd = format!("0.{}e{}", chars, e);
        // try!(w.write_all(self.as_bytes()));
        // w.write_all(format!("{}", self).as_bytes());
        println!("out-val: {}", self);
        try!(write!(out, "{}", self));

        Ok(IsNull::No)
    }

    accepts!(Type::Numeric);
    to_sql_checked!();
}
