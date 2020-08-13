#![allow(unused)]
mod dbutils;
mod parse;

use std::str::FromStr;
use rusqlite::{Connection, Result, NO_PARAMS};
use parse::*;

fn main() -> Result<()> {
    let mut conn = Connection::open("theta.db")?;
    //let mut conn2 = Connection::open("theta.db")?;
    let trades = vec![("PRPL200831C30", 1, 2.00, "sell"), ("MU200814C50", 2, 0.70, "sell")];
    dbutils::create_tables(&conn);
    for t in trades {
        let mut legs: Vec<Leg> = Vec::new();
        legs.push(
            Leg::new(Opt::from_str(t.0).unwrap(),  t.1,  t.2,
                     String::from(t.3),  0.0,  0.65)
        );
        let tr = Trade {
            date: String::from("2020-08-07"),
            strategy: String::from("Covered Call"),
            legs };

        dbutils::commit_trade(&mut conn, tr);
    }
    //println!("{:#?}", trade);
    Ok(())
}

//SELL 2 AMD200701C53 .70
//ROLL 1 AMD200701C53 AMD200708C53 
