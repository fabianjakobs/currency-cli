use std::{collections::HashMap, str::FromStr};

use xmltree::{Element, XMLNode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Currency {
    USD,
    JPY,
    BGN,
    CZK,
    DKK,
    GBP,
    HUF,
    PLN,
    RON,
    SEK,
    CHF,
    ISK,
    NOK,
    HRK,
    TRY,
    AUD,
    BRL,
    KRW,
    MXN,
    MYR,
    NZD,
    PHP,
    SGD,
    THB,
    ZAR,
    CAD,
    CNY,
    HKD,
    IDR,
    ILS,
    INR,
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(Currency::USD),
            "JPY" => Ok(Currency::JPY),
            "BGN" => Ok(Currency::BGN),
            "CZK" => Ok(Currency::CZK),
            "DKK" => Ok(Currency::DKK),
            "GBP" => Ok(Currency::GBP),
            "HUF" => Ok(Currency::HUF),
            "PLN" => Ok(Currency::PLN),
            "RON" => Ok(Currency::RON),
            "SEK" => Ok(Currency::SEK),
            "CHF" => Ok(Currency::CHF),
            "ISK" => Ok(Currency::ISK),
            "NOK" => Ok(Currency::NOK),
            "HRK" => Ok(Currency::HRK),
            "TRY" => Ok(Currency::TRY),
            "AUD" => Ok(Currency::AUD),
            "BRL" => Ok(Currency::BRL),
            "KRW" => Ok(Currency::KRW),
            "MXN" => Ok(Currency::MXN),
            "MYR" => Ok(Currency::MYR),
            "NZD" => Ok(Currency::NZD),
            "PHP" => Ok(Currency::PHP),
            "SGD" => Ok(Currency::SGD),
            "THB" => Ok(Currency::THB),
            "ZAR" => Ok(Currency::ZAR),
            "CAD" => Ok(Currency::CAD),
            "CNY" => Ok(Currency::CNY),
            "HKD" => Ok(Currency::HKD),
            "IDR" => Ok(Currency::IDR),
            "ILS" => Ok(Currency::ILS),
            "INR" => Ok(Currency::INR),
            _ => Err(format!("'{}' is not a valid value for Currency", s)),
        }
    }
}

impl Currency {}

fn parse_currencies(
    body: &str,
) -> Result<HashMap<Currency, f64>, Box<dyn std::error::Error + Send + Sync>> {
    let document = Element::parse(body.as_bytes()).unwrap();

    let conversion_map: HashMap<_, _> = document
        .get_child("Cube")
        .unwrap()
        .get_child("Cube")
        .unwrap()
        .children
        .iter()
        .map(|node| {
            let attributes = &node.as_element().unwrap().attributes;
            (
                Currency::from_str(&attributes["currency"]).unwrap(),
                attributes["rate"].parse::<f64>().unwrap(),
            )
        })
        .collect();

    println!("Doc: {:?}", conversion_map);
    Ok(conversion_map)
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let uri = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";

    let body: String = ureq::get(uri).call()?.into_string()?;

    let conversion_map = parse_currencies(&body)?;
    println!("Response: {:?}", conversion_map);

    // USD to EUR
    println!(
        "100 USD equals {:.2} EUR",
        100.0 / conversion_map[&Currency::USD]
    );

    Ok(())
}
