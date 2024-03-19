use std::fs::File;
use serde::Deserialize;
use std::error::Error;
use std::io::{BufRead, BufReader};
use csv::ReaderBuilder;

#[derive(Debug, Default, Deserialize)]
struct InterconnectorData {
    row_type: String,
    file_type: String,
    file_subtype: String,
    file_descriptor: String,
    settlement_date: String,
    run_no: u32,
    interconnector_id: String,
    period_id: u32,
    metered_mw_flow: f64,
    mw_flow: f64,
    mw_losses: f64,
    last_changed: String,
}

#[derive(Debug, Default, Deserialize)]
struct PriceData {
    // Fields as before
    row_type: String,
    file_type: String,
    file_subtype: String,
    file_descriptor: String,
    settlement_date: String,
    run_no: u32,
    region_id: String,
    period_id: u32,
    rrp: f64,
    eep: f64,
    invalid_flag: u32,
    last_changed: String,
    rop: f64,
    raise6sec_rrp: f64,
    raise6sec_rop: f64,
    raise60sec_rrp: f64,
    raise60sec_rop: f64,
    raise5min_rrp: f64,
    raise5min_rop: f64,
    raisereg_rrp: f64,
    raisereg_rop: f64,
    lower6sec_rrp: f64,
    lower6sec_rop: f64,
    lower60sec_rrp: f64,
    lower60sec_rop: f64,
    lower5min_rrp: f64,
    lower5min_rop: f64,
    lowerreg_rrp: f64,
    lowerreg_rop: f64,
    raise1sec_rrp: f64,
    raise1sec_rop: f64,
    lower1sec_rrp: f64,
    lower1sec_rop: f64,
    price_status: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RecordCurrentTradingIs {
    Variant1(InterconnectorData),
    Variant2(PriceData),
}

trait ProcessRecord<T> {
    fn process(line: &str) -> Result<T, Box<dyn Error>>;
}

impl ProcessRecord<RecordCurrentTradingIs> for InterconnectorData {
    fn process(line: &str) -> Result<RecordCurrentTradingIs, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(line.as_bytes());
        let record = rdr.deserialize::<InterconnectorData>().next().ok_or("No record found")??;
        Ok(RecordCurrentTradingIs::Variant1(record))
    }
}

impl ProcessRecord<RecordCurrentTradingIs> for PriceData {
    fn process(line: &str) -> Result<RecordCurrentTradingIs, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(line.as_bytes());
        let record = rdr.deserialize::<PriceData>().next().ok_or("No record found")??;
        Ok(RecordCurrentTradingIs::Variant2(record))
    }
}

fn process_file_current_trading_is(path: &str) -> Result<Vec<RecordCurrentTradingIs>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records: Vec<RecordCurrentTradingIs> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        match line.chars().next() {
            // Skip lines starting with 'C' or 'I'
            // TODO: add Record for first line starting with C
            // TODO: add header validation since some fields may be missing
            Some('C') | Some('I') => continue,
            Some('D') => {
                if line.starts_with("D,TRADING,INTERCONNECTORRES") {
                    records.push(InterconnectorData::process(&line)?);
                } else if line.starts_with("D,TRADING,PRICE") {
                    records.push(PriceData::process(&line)?);
                } else {
                    return Err("Unknown record type".into());
                }
            }
            _ => return Err("Invalid line format".into()),
        }
    }

    Ok(records)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "src/fixtures/PUBLIC_TRADINGIS_202403031335_0000000412683134.CSV";
    let records = process_file_current_trading_is(path)?;

    // Debug print to verify parsing
    for record in records {
        println!("{:?}", record);
    }

    Ok(())
}
