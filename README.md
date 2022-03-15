# currency-cli

CLI to convert currencies

```
currency --from EUR --to USD 12.13
```

Uses data from the European Central Bank https://www.ecb.europa.eu/stats/eurofxref/eurofxref-daily.xml

## Compiling a static binary

Currently only works for x86

```sh
cargo build --release --target=x86_64-unknown-linux-musl
```