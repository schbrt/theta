use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug )]
pub struct Opt {
    pub symbol: String,
    pub expiration: String,
    pub strike: f64,
    pub kind: String,
    pub price: f64,
}

pub struct Leg {
    pub opt: Opt,
    pub num_contracts: i32,
    pub purchase_date: String,
    pub buy_sell: String,
    commission: f64,
    per_contract: f64
}

impl Leg {
    pub fn value(&self) -> f64 {
        let buy_sell_mult = 
            match self.buy_sell.as_str() {
                "buy"  => -1.0,
                "sell" => 1.0,
                _ => panic!("Transaction type must be buy or sell.")
        };
        buy_sell_mult * self.num_contracts as f64 * ((self.opt.price * 100.0) - self.per_contract) - self.commission
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

// Takes a conventional option string and extracts the symbol, expiration, type, and strike.
// Attaches price and contract commission.
pub fn create_opt(s: &str, price: f64, purchase_date: &str, buy_sell: &str) -> Opt {
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
        //purchase_date: String::from(purchase_date),
        price,
        //buy_sell: String::from(buy_sell)
    };
    return opt;
}