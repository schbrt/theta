#[macro_use]
use crate::parse::*;
use rusqlite::{Connection, Result, NO_PARAMS, params};

fn create_security_table(conn: &Connection) {
    conn.execute(r#"CREATE TABLE IF NOT EXISTS securities (
                id INTEGER PRIMARY KEY,
                symbol TEXT NOT NULL,
                expiration TEXT NOT NULL,
                strike REAL NOT NULL,
                kind TEXT NOT NULL,
                UNIQUE(symbol, expiration, strike, kind))"#, NO_PARAMS);
}

fn create_transaction_table(conn: &Connection) {
    conn.execute(r#"CREATE TABLE IF NOT EXISTS transactions (
                legid INTEGER PRIMARY KEY AUTOINCREMENT,
                securityid INT,
                txid INT,
                num_contracts INT NOT NULL, 
                value REAL NOT NULL,
                price REAL NOT NULL,
                FOREIGN KEY(txid) REFERENCES txids(txid)
                FOREIGN KEY(securityid) REFERENCES securities(id))"#, NO_PARAMS);
}

fn create_transaction_id_table(conn: &Connection) {
   conn.execute(r#"CREATE TABLE IF NOT EXISTS txids (
                txid INTEGER PRIMARY KEY AUTOINCREMENT,
                strategy TEXT,
                date TEXT)"#, NO_PARAMS);
}

pub fn create_tables(conn: &Connection) {
    create_transaction_table(conn);
    create_security_table(conn);
    create_transaction_id_table(conn);
}

// Use sqlite transaction to commit an option transaction. A simple transaction would be buying
// or selling one or more contracts. Advanced option trades can be broken down into a series of buys
// and sells.
pub fn commit_trade(conn: &mut Connection, trade: Trade) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(r#"INSERT INTO txids
                    (strategy, date)
                    VALUES (?1, ?2)"#,
                    params![trade.strategy, trade.date])?; 
    // Get sqlite rowid for the overall stock transaction (multiple legs)
    let transaction_last_rowid = tx.last_insert_rowid();
    for leg in trade.legs {
        println!("{:#?}", leg);
        tx.execute(r#"INSERT OR IGNORE INTO securities 
                        (symbol, expiration, strike, kind)
                        VALUES (?1, ?2, ?3, ?4)"#,
                        params![leg.opt.symbol, leg.opt.expiration, leg.opt.strike,
                                leg.opt.kind])?;
        // Get sqlite rowid for the security involved in current leg
        let last_rowid = tx.last_insert_rowid();
        tx.execute(r#"INSERT INTO transactions
                        (securityid, txid, num_contracts, value, price)
                        VALUES (?1, ?2, ?3, ?4, ?5)"#, 
                        params![last_rowid, transaction_last_rowid, leg.num_contracts, 
                        leg.value(), leg.price])?;
    }
    tx.commit();
    Ok(())
}