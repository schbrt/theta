use lazy_static::lazy_static;
use std::collections::HashMap;
use std::error::Error;
//use structopt::StructOpt;
use rusqlite::{Connection, Result};
use regex::Regex;

/*    fn create_table(conn: &mut Connection) {
conn.execute("create table if not exists transactions (
                    id integer primary key,
                    premium real not null,
                                                          )");

}*/

struct ParseError {
    details: String
}

#[derive(Debug)]
struct Opt {
    symbol: String,
    expiration: String,
    strike: f32,
    kind: String,
    price: f32,
}

fn create_option_struct(s: &str, p: f32) -> Opt {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
                (?P<symbol>^[A-Z]{1,6})
                (?P<expiration>\d{6})
                (?P<type>[C,P])
                (?P<strike>[\d\.]*$)").unwrap();
    }
    let parsed_option = RE.captures(s).unwrap();
    let opt = Opt{
        symbol: parsed_option.name("symbol").expect("Option parsing error").as_str().to_string(),
        expiration: parsed_option.name("expiration").expect("Option parsing error").as_str().to_string(),
        strike: parsed_option.name("strike").expect("Option parsing error").as_str().parse().unwrap(),
        kind: parsed_option.name("type").expect("Option parsing error").as_str().to_string(),
        price: p
    };
    return opt;
}

fn main() {
    //let mut conn = Connection::open("theta.db")?;
    let opts = vec![("AMD200626C53", 0.70), ("AMAT200619C55", 1.00), ("MSFT200529C182.5", 2.40)];
    for opt in opts {
        let out = create_option_struct(opt.0, opt.1);
        println!("{:#?}", out);
    }
}
