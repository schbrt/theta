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
        let o = Opt::new(opt.0);
        txs.push(o);
    }
    println!("{:#?}", txs);
    Ok(())
}

//SELL 2 AMD200701C53 .70
//ROLL 1 AMD200701C53 AMD200708C53 
