use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Opt {
    pub symbol: String,
    pub expiration: String,
    pub strike: f64,
    pub kind: String,
}

#[derive(Debug)]
pub struct Leg {
    pub opt: Opt,
    pub num_contracts: i32,
    pub buy_sell: String,
    pub price: f64,
    commission: f64,
    per_contract: f64
}

#[derive(Debug)]
pub struct Trade {
    pub date: String,
    pub strategy: String,
    pub legs: Vec<Leg>
}

impl Opt {
    // Creates a new Opt struct from an option string of the form AMD200701C53
    pub fn new(opt_str: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                    (?P<symbol>^[A-Z]{1,6})
                    (?P<expiration>\d{6})
                    (?P<type>[C,P])
                    (?P<strike>[\d\.]*$)").unwrap();
        }
        let parsed_option = RE.captures(opt_str).unwrap();
        return Opt {
            symbol: parsed_option.name("symbol").expect("Option parsing error").as_str().to_string(),
            expiration: convert_expir_date(&parsed_option.name("expiration").expect("Option parsing error").as_str().to_string()),
            strike: parsed_option.name("strike").expect("Option parsing error").as_str().parse().unwrap(),
            kind: parsed_option.name("type").expect("Option parsing error").as_str().to_string(),
        };
    }
}

impl Leg {
    pub fn value(&self) -> f64 {
        let buy_sell_mult = 
            match self.buy_sell.as_str() {
                "buy"  => -1.0,
                "sell" => 1.0,
                _ => unreachable!(r#"buy_sell can only be "buy" or "sell"."#)
        };
        buy_sell_mult * self.num_contracts as f64 * ((self.price * 100.0) - self.per_contract) - self.commission
    }
}

impl Trade {
    pub fn value(&self) -> f64 {
        self.legs.iter().map(|l|{
            l.value()    
        }).sum()
    }
}

// OPTyyddmmT## -> yyyy-mm-dd
// AMD200701C55: expiration date string is 200701
// 200701 -> 2020-07-01
fn convert_expir_date(d: &str) -> String {
    let char_vec: Vec<char> = d.chars().collect();
    let year = String::from("20") + &char_vec[0..=1].to_vec().into_iter().collect::<String>();
    let month = char_vec[2..=3].to_vec().into_iter().collect::<String>();
    let day = char_vec[4..=5].to_vec().into_iter().collect::<String>();
    format!("{}-{}-{}", year, month, day)
}