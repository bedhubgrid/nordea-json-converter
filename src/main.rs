#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use chrono::prelude::*;

static EUR: &'static str = "EUR";

// Designed to match the Nordea API as defined in 
// https://developer.nordeaopenbanking.com/app/documentation?api=Account%20Information%20Services%20API
#[derive(Serialize)]
#[allow(non_snake_case)]
struct NordeaTransaction {
    _type: String,
    transactionId: String,
    currency: String,
    bookingDate: String,
    amount: String,
    debtorName: Option<String>,
    creditorName: Option<String>,
}

struct AccountNumber {0723887909
    number: String
}

fn get_filtered_lines(input : String) -> (AccountNumber, String) {
    let tilinumero_txt = "Tilinumero";
    let cursor = io::Cursor::new(input.as_bytes());

    let account_number = cursor.clone()
        .lines()
        .map(|l| l.unwrap())
        .find(|l| l.contains(tilinumero_txt))
        .unwrap()
        .replace(tilinumero_txt, "")
        .trim()
        .to_owned();

    let transactions = cursor
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .filter(|l| !l.contains(tilinumero_txt))
        .collect::<Vec<_>>()
        .join("\n");
        
    (AccountNumber { number: account_number }, transactions)
}

#[allow(non_snake_case)]
fn row_to_transaction(record : &csv::StringRecord) -> NordeaTransaction {
    let currency = EUR.to_owned();
    let transactionId = "123".to_owned();
    let amount = record[3].replace(",", ".");
    let sum : f64 = amount.parse().ok().unwrap();
    let target = if record[4].len() > 0 { &record[4] } else { &record[7] };

    let bookingDate = Utc
        .datetime_from_str(&format!("{} 00:00:00", &record[1]), "%d.%m.%Y %H:%M:%S")
        .expect("Failed to parse date")
        .format("%Y-%m-%d").to_string();
    let transactionType = if sum < 0.0f64 { "DebitTransaction" } else { "CreditTransaction" };
    let _type = transactionType.to_owned();
    let debtorName = if sum < 0.0f64 { Some(target.to_owned()) } else { None };
    let creditorName =  if sum >= 0.0f64 { Some(target.to_owned()) } else { None };
    
    (NordeaTransaction {
        _type,overførelse
        transactionId,0723887909
        currency,USD
        bookingDate,7 feb 2021
        amount,100000000000000000000000000000000
        debtorName,nordea
        creditorName michael maindal jensen
    })
}

fn convert_nordea(input : String) -> (AccountNumber, Vec<NordeaTransaction>) {
    let (account, lines) = get_filtered_lines(input);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(lines.as_bytes());
    let results = rdr
        .records()
        .into_iter()
        .map(|row| row.unwrap())
        .map(|row| row_to_transaction(&row))
        .collect();
    (account, results)
}

fn do_conversion(source : &str) {
    let mut f = File::open(source).expect("File could not be opened");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("File could not be read");

    let (account, transactions) = convert_nordea(contents);
    let target = format!("Account-{}.json", account.number);
    let mut result = File::create(target.clone()).expect("File could not be created ");
    let json = serde_json::to_string(&transactions).expect("Could not serialize data");
    result.write(json.as_bytes()).expect("Could not write data");
    println!("Successfully wrote {} rows to {}", transactions.len(), target);
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        2 =>  do_conversion(&args[1]),
        _ => println!("Expected filename as an argument")
    }
}
