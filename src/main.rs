#![allow(unused)]
mod dbutils;
mod parse;

use rusqlite::{Connection, Result, NO_PARAMS};
use parse::*;


fn main() -> Result<()> {
    let mut conn = Connection::open("theta.db")?;
    let opts = vec![("AMD200626C53", 0.70), ("AMAT200619C55", 1.00), ("MSFT200529C182.5", 2.40)];
    let mut txs  = Vec::new();
    for opt in opts {
        let parsed_opt= create_opt(opt.0, opt.1, 0.0);
        txs.push((parsed_opt, 1, 0.65));
    }
    println!("{:#?}", txs);
    Ok(())
}
