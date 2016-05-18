extern crate gmp;

use std::io::prelude::*;

use self::gmp::mpf::Mpf;
use types::{FromSql, ToSql, Type, IsNull, SessionInfo};
use Result;

impl FromSql for Mpf {
    fn from_sql<R: Read>(_: &Type, raw: &mut R, _: &SessionInfo) -> Result<Mpf> {
        // let mut bytes = [0; 16];
        // try!(raw.read_exact(&mut bytes));
        Ok(Mpf::new(32))
    }

    accepts!(Type::Numeric);
}

impl ToSql for Mpf {
    fn to_sql<W: Write + ?Sized>(&self, _: &Type, w: &mut W, _: &SessionInfo) -> Result<IsNull> {
        // try!(w.write_all(self.as_bytes()));
        Ok(IsNull::No)
    }

    accepts!(Type::Numeric);
    to_sql_checked!();
}
