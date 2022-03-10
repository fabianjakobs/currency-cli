use std::collections::HashMap;

use serde::Deserialize;
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "gesmes:Envelope")]
struct Document {
    #[serde(rename = "Cube")]
    cube: Cube1,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Cube1 {
    #[serde(rename = "Cube")]
    cube: Cube2,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Cube2 {
    time: String,
    #[serde(rename = "Cube")]
    conversion: Vec<Conversion>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Cube")]
struct Conversion {
    currency: Currency,
    rate: f64,
}

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
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

impl Currency {}

fn parse_currencies(
    body: &str,
) -> Result<HashMap<Currency, f64>, Box<dyn std::error::Error + Send + Sync>> {
    let document: Document = from_str(body)?;
    let conversions = document.cube.cube.conversion;

    let conversion_map: HashMap<_, _> = conversions.iter().map(|c| (c.currency, c.rate)).collect();
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
