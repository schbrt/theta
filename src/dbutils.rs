#[macro_use]
use crate::parse::*;
use rusqlite::{Connection, Result, NO_PARAMS, params};

pub fn create_table(conn: &mut Connection) {
    conn.execute(r#"create table if not exists transactions (
                    id integer primary key,
                    symbol text not null,
                    expiration text not null,
                    strike real not null,
                    kind text not null,
                    price real not null,
                    num_contracts int not null, 
                    value real not null
                    strategy text)"#, NO_PARAMS);
}

// Use sqlite transaction to commit an option transaction. A simple transaction would be buying
// or selling one or more contracts. Advanced option trades can be broken down into a series of buys
// and sells.
pub fn commit_transaction(conn: &mut Connection, legs: Vec<Leg>, strategy: &str) -> Result<()> {
    let tx = conn.transaction()?;
    for leg in legs {
        tx.execute(r#"INSERT INTO transactions
                        (symbol, expiration, strike, kind, price, num_contracts, value, strategy)
                        VALUES (values)"#, 
                        params![leg.opt.symbol, leg.opt.expiration, leg.opt.strike,
                                leg.opt.kind, leg.opt.price, leg.num_contracts,
                                leg.value(), strategy])?;
    }
    tx.commit()
}