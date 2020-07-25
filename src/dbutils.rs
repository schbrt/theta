use crate::parse::*;
use rusqlite::{Connection, Result, NO_PARAMS};

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
pub fn commit_transactions(conn: &mut Connection, transactions: Vec<Opt>) -> Result<()>{
    let tx = conn.transaction()?;
    for leg in transactions {
        tx.execute("INSERT INTO transactions
                        (symbol, expiration, strike, kind, price, value)
                        VALUES (values)",
                         &[t.])?;
    }
    tx.commit()
}