use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Opt {
    symbol: String,
    expiration: String,
    strike: f32,
    kind: String,
    price: f32,
}

pub struct Transaction {
    legs: Vec<(Opt, u32)>,
    commission: f32
}

impl Transaction{
    fn value(&self, price: f32) {
        _(price * self.num_contracts as f32 * 100.0) - self.commission;
    }
}

// AMD200701C55: expiration date string is 200701
// 200701 -> 2020-07-01
fn convert_expir_date(d: &str) -> String {
    let char_vec: Vec<char> = d.chars().collect();
    let year= char_vec[0];
    let month = char_vec[1];
    let day= char_vec[2];
    format!("{}-{}-{}", year, month, day)
}

pub fn parse_opt_string(s: &str, p: f32) -> Opt {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
                (?P<symbol>^[A-Z]{1,6})
                (?P<expiration>\d{6})
                (?P<type>[C,P])
                (?P<strike>[\d\.]*$)").unwrap();
    }
    let parsed_option = RE.captures(s).unwrap();
    let opt = Opt {
        symbol: parsed_option.name("symbol").expect("Option parsing error").as_str().to_string(),
        expiration: convert_expir_date(&parsed_option.name("expiration").expect("Option parsing error").as_str().to_string()),
        strike: parsed_option.name("strike").expect("Option parsing error").as_str().parse().unwrap(),
        kind: parsed_option.name("type").expect("Option parsing error").as_str().to_string(),
        price: p
    };
    return opt;
}