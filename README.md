# Nordea CSV to Nordea API JSON converter

This is a small utility to convert Nordea online bank CSV export to JSON format specified by the Nordea API. See the [Nordea Open Banking swagger.json](https://developer.nordeaopenbanking.com/sites/sandbox-obi-dev.developer.eu.apiconnect.ibmcloud.com/files/api-docs/ais/v2/files/swagger.json).

## 1. Install rust
To install Rust, run the following in your terminal, then follow the onscreen instructions.

```bash
curl https://sh.rustup.rs -sSf | sh
```

## 2. Export Nordea data

Log in to the bank and do a CSV export of account transactions.

## 3. Build this tool

Clone the source code to a directory and run `cargo build --release`.

## 4. Run the tool

The tool is built into folder `target/release`.

Running the tool
`target/release/nordea-json-converter Tapahtumat_FI6593857450293470_20171204_20171205.csv`

If the conversion succeeds, a output file is created in the directory the tool was run in.