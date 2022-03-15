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
    let body = ureq::get(uri).call()?;
    let document = Element::parse(body.into_reader())?;

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
