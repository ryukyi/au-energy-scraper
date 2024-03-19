use std::fmt;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Deserialize, Default)]
struct InterconnectorData {
    #[serde(rename = "ROW_TYPE")]
    row_type: String,
    #[serde(rename = "FILE_TYPE")]
    file_type: String,
    #[serde(rename = "FILE_SUBTYPE")]
    file_subtype: String,
    #[serde(rename = "FILE_DESCRIPTOR")]
    file_descriptor: String,
    #[serde(rename = "SETTLEMENTDATE")]
    settlement_date: Option<String>,
    #[serde(rename = "RUNNO")]
    run_no: Option<u32>,
    #[serde(rename = "INTERCONNECTORID")]
    interconnector_id: Option<String>,
    #[serde(rename = "PERIODID")]
    period_id: Option<u32>,
    #[serde(rename = "METEREDMWFLOW")]
    metered_mw_flow: Option<f64>,
    #[serde(rename = "MWFLOW")]
    mw_flow: Option<f64>,
    #[serde(rename = "MWLOSSES")]
    mw_losses: Option<f64>,
    #[serde(rename = "LASTCHANGED")]
    last_changed: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct PriceData {
    #[serde(rename = "ROW_TYPE")]
    row_type: String,
    #[serde(rename = "FILE_TYPE")]
    file_type: String,
    #[serde(rename = "FILE_SUBTYPE")]
    file_subtype: String,
    #[serde(rename = "FILE_DESCRIPTOR")]
    file_descriptor: String,
    #[serde(rename = "SETTLEMENT_DATE")]
    settlement_date: Option<String>,
    #[serde(rename = "RUN_NO")]
    run_no: Option<u32>,
    #[serde(rename = "REGION_ID")]
    region_id: Option<String>,
    #[serde(rename = "PERIOD_ID")]
    period_id: Option<u32>,
    #[serde(rename = "RRP")]
    rrp: Option<f64>,
    #[serde(rename = "EEP")]
    eep: Option<f64>,
    #[serde(rename = "INVALID_FLAG")]
    invalid_flag: Option<u32>,
    #[serde(rename = "LAST_CHANGED")]
    last_changed: Option<String>,
    #[serde(rename = "ROP")]
    rop: Option<f64>,
    #[serde(rename = "RAISE6SEC_RRP")]
    raise6sec_rrp: Option<f64>,
    #[serde(rename = "RAISE6SEC_ROP")]
    raise6sec_rop: Option<f64>,
    #[serde(rename = "RAISE60SEC_RRP")]
    raise60sec_rrp: Option<f64>,
    #[serde(rename = "RAISE60SEC_ROP")]
    raise60sec_rop: Option<f64>,
    #[serde(rename = "RAISE5MIN_RRP")]
    raise5min_rrp: Option<f64>,
    #[serde(rename = "RAISE5MIN_ROP")]
    raise5min_rop: Option<f64>,
    #[serde(rename = "RAISEREG_RRP")]
    raisereg_rrp: Option<f64>,
    #[serde(rename = "RAISEREG_ROP")]
    raisereg_rop: Option<f64>,
    #[serde(rename = "LOWER6SEC_RRP")]
    lower6sec_rrp: Option<f64>,
    #[serde(rename = "LOWER6SEC_ROP")]
    lower6sec_rop: Option<f64>,
    #[serde(rename = "LOWER60SEC_RRP")]
    lower60sec_rrp: Option<f64>,
    #[serde(rename = "LOWER60SEC_ROP")]
    lower60sec_rop: Option<f64>,
    #[serde(rename = "LOWER5MIN_RRP")]
    lower5min_rrp: Option<f64>,
    #[serde(rename = "LOWER5MIN_ROP")]
    lower5min_rop: Option<f64>,
    #[serde(rename = "LOWERREG_RRP")]
    lowerreg_rrp: Option<f64>,
    #[serde(rename = "LOWERREG_ROP")]
    lowerreg_rop: Option<f64>,
    #[serde(rename = "RAISE1SEC_RRP")]
    raise1sec_rrp: Option<f64>,
    #[serde(rename = "RAISE1SEC_ROP")]
    raise1sec_rop: Option<f64>,
    #[serde(rename = "LOWER1SEC_RRP")]
    lower1sec_rrp: Option<f64>,
    #[serde(rename = "LOWER1SEC_ROP")]
    lower1sec_rop: Option<f64>,
    #[serde(rename = "PRICE_STATUS")]
    price_status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RecordCurrentTradingIs {
    Variant1(InterconnectorData),
    Variant2(PriceData),
}

impl fmt::Display for InterconnectorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InterconnectorData: row_type: {}, file_type: {}, file_subtype: {}, file_descriptor: {}, settlement_date: {:?}, run_no: {:?}, interconnector_id: {:?}, period_id: {:?}, metered_mw_flow: {:?}, mw_flow: {:?}, mw_losses: {:?}, last_changed: {:?}",
            self.row_type, self.file_type, self.file_subtype, self.file_descriptor,
            self.settlement_date, self.run_no, self.interconnector_id, self.period_id,
            self.metered_mw_flow, self.mw_flow, self.mw_losses, self.last_changed)
    }
}

// Implement Display for PriceData
impl fmt::Display for PriceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PriceData: row_type: {}, file_type: {}, file_subtype: {}, file_descriptor: {}, settlement_date: {:?}, run_no: {:?}, region_id: {:?}, period_id: {:?}, rrp: {:?}, eep: {:?}, invalid_flag: {:?}, last_changed: {:?}, rop: {:?}, raise6sec_rrp: {:?}, raise6sec_rop: {:?}, raise60sec_rrp: {:?}, raise60sec_rop: {:?}, raise5min_rrp: {:?}, raise5min_rop: {:?}, raisereg_rrp: {:?}, raisereg_rop: {:?}, lower6sec_rrp: {:?}, lower6sec_rop: {:?}, lower60sec_rrp: {:?}, lower60sec_rop: {:?}, lower5min_rrp: {:?}, lower5min_rop: {:?}, lowerreg_rrp: {:?}, lowerreg_rop: {:?}, raise1sec_rrp: {:?}, raise1sec_rop: {:?}, lower1sec_rrp: {:?}, lower1sec_rop: {:?}, price_status: {:?}",
            self.row_type, self.file_type, self.file_subtype, self.file_descriptor,
            self.settlement_date, self.run_no, self.region_id, self.period_id,
            self.rrp, self.eep, self.invalid_flag, self.last_changed,
            self.rop, self.raise6sec_rrp, self.raise6sec_rop, self.raise60sec_rrp, self.raise60sec_rop,
            self.raise5min_rrp, self.raise5min_rop, self.raisereg_rrp, self.raisereg_rop,
            self.lower6sec_rrp, self.lower6sec_rop, self.lower60sec_rrp, self.lower60sec_rop,
            self.lower5min_rrp, self.lower5min_rop, self.lowerreg_rrp, self.lowerreg_rop,
            self.raise1sec_rrp, self.raise1sec_rop, self.lower1sec_rrp, self.lower1sec_rop,
            self.price_status)
    }
}

// Implement Display for RecordCurrentTradingIs
impl fmt::Display for RecordCurrentTradingIs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordCurrentTradingIs::Variant1(data) => write!(f, "{}", data),
            RecordCurrentTradingIs::Variant2(data) => write!(f, "{}", data),
        }
    }
}

trait ProcessRecord<T> {
    fn process(line: &str) -> Result<T, Box<dyn Error>>;
}

impl ProcessRecord<RecordCurrentTradingIs> for InterconnectorData {
    fn process(line: &str) -> Result<RecordCurrentTradingIs, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let record = rdr
            .deserialize::<InterconnectorData>()
            .next()
            .ok_or("No record found")??;
        Ok(RecordCurrentTradingIs::Variant1(record))
    }
}

impl ProcessRecord<RecordCurrentTradingIs> for PriceData {
    fn process(line: &str) -> Result<RecordCurrentTradingIs, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let record = rdr
            .deserialize::<PriceData>()
            .next()
            .ok_or("No record found")??;
        Ok(RecordCurrentTradingIs::Variant2(record))
    }
}

fn process_file_current_trading_is(
    path: &str,
) -> Result<Vec<RecordCurrentTradingIs>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut records: Vec<RecordCurrentTradingIs> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with("I,TRADING,INTERCONNECTORRES") || line.starts_with("I,TRADING,PRICE") {
            println!("{:?}", &line);
        }
        match line.chars().next() {
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
    for record in records {
        println!("{}", record);
    }

    Ok(())
}
