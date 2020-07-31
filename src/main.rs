#![allow(unused)]
mod dbutils;
mod parse;

use std::str::FromStr;
use rusqlite::{Connection, Result, NO_PARAMS};
use parse::*;

fn main() -> Result<()> {
    let conn = Connection::open("theta.db")?;
    let opts = vec![("AMD200626C53", 2, 0.7, "sell"), ("AMAT200619C53", 2, 0.01, "buy")];
    let mut legs: Vec<Leg> = Vec::new();
    dbutils::create_tables(&conn);
    for opt in opts {
        legs.push(
            Leg::new(Opt::from_str(opt.0).unwrap(),  opt.1,  opt.2,
                     String::from(opt.3),  0.0,  0.65)
        );
    }
    let trade = Trade {
                date: String::from("2020-06-19"),
                strategy: String::from("Roll"),
                legs };
    println!("{:#?}", trade);
    println!("{}", trade.value());
    Ok(())
}

//SELL 2 AMD200701C53 .70
//ROLL 1 AMD200701C53 AMD200708C53 
