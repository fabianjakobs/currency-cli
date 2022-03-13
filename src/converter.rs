use core::fmt;
use std::{collections::HashMap, str::FromStr};
use xmltree::Element;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum Currency {
    EUR,
    USD,
    JPY,
    BGN,
    CZK,
    DKK,
    GBP,
    HUF,
    RON,
    PLN,
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

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "EUR" => Ok(Currency::EUR),
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

#[derive(Debug)]
pub struct Converter {
    conversion_map: HashMap<Currency, f64>,
}

impl Converter {
    pub fn new(conversion_map: HashMap<Currency, f64>) -> Converter {
        Converter { conversion_map }
    }

    pub fn from_xml(
        document: &Element,
    ) -> Result<Converter, Box<dyn std::error::Error + Send + Sync>> {
        let mut conversion_map: HashMap<_, _> = document
            .get_child("Cube")
            .ok_or("Invalid XML")?
            .get_child("Cube")
            .ok_or("Invalid XML")?
            .children
            .iter()
            .map(|node| {
                let attributes = &node.as_element().ok_or("Invalid XML")?.attributes;
                Ok((
                    Currency::from_str(&attributes["currency"])?,
                    attributes["rate"].parse::<f64>()?,
                ))
            })
            .collect::<Result<HashMap<_, _>, Box<dyn std::error::Error + Send + Sync>>>()?;

        conversion_map.insert(Currency::EUR, 1.0);

        Ok(Converter { conversion_map })
    }

    pub fn convert(&self, value: f64, from: Currency, to: Currency) -> f64 {
        if from == Currency::EUR {
            value * self.conversion_map[&to]
        } else if to == Currency::EUR {
            value / self.conversion_map[&from]
        } else {
            (value / self.conversion_map[&from]) * self.conversion_map[&to]
        }
    }
}
