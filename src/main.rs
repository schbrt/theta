#![allow(unused)]
mod dbutils;
mod parse;

use rusqlite::{Connection, Result, NO_PARAMS};
use parse::*;


fn main() -> Result<()> {
    let mut conn = Connection::open("theta.db")?;
    let opts = vec![("AMD200626C53", 0.70), ("AMAT200619C55", 1.00), ("MSFT200529C182.5", 2.40)];
    let mut txs: Vec<Transaction> = Vec::new();
    for opt in opts {
        let out = parse_opt_string(opt.0, opt.1);
        txs.push(Transaction{option: out, num_contracts: 1, commission: 0.65});
        //println!("{:#?}", out);
    }
    println!("{:?}", txs.len());
    Ok(())
}
