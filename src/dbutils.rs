#[macro_use]
use crate::parse::*;
use rusqlite::{Connection, Result, NO_PARAMS, params};

pub fn create_table(conn: &mut Connection) {
    conn.execute("create table if not exists transactions (
                    id integer primary key,
                    symbol text not null,
                    expiration text not null,
                    strike real not null,
                    kind text not null,
                    price real not null,
                    value real not null)", NO_PARAMS);

}

// Use sqlite transaction to commit an option transaction. A simple transaction would be buying
// or selling one or more contracts. Advanced option trades can be broken down into a series of buys
// and sells.
pub fn commit_transaction(conn: &mut Connection, legs: Vec<Opt>, commission: f64) -> Result<()> {
    let tx = conn.transaction()?;
    for leg in legs {
        tx.execute("INSERT INTO transactions
                        (symbol, expiration, strike, kind, price, value)
                        VALUES (values)", 
                        params![leg.symbol, leg.expiration, leg.strike, leg.kind, leg.price])?;
    }
    tx.commit()
}