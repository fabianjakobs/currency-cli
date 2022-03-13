mod converter;

use clap::Parser;
use converter::{Converter, Currency};
use xmltree::Element;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    from: Currency,
    #[clap(short, long)]
    #[clap(default_value_t = Currency::EUR)]
    to: Currency,
    value: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();

    let uri = "https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml";
    let body: String = ureq::get(uri).call()?.into_string()?;
    let document = Element::parse(body.as_bytes())?;

    let converter = Converter::from_xml(&document)?;

    println!(
        "{} {:?} equals {:.2} {:?}",
        args.value,
        args.from,
        converter.convert(args.value, args.from, args.to),
        args.to
    );

    Ok(())
}
