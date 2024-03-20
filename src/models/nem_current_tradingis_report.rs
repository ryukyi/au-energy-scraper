use chrono::{DateTime, Utc};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::io::{BufRead, BufReader};

use crate::common::parser_types::ProcessRecord;
use crate::time_utils::datetimezone_conversion::deserialize_sydney_datetime_to_utc;

#[derive(Debug, Deserialize, Default)]
pub struct InterconnectorData {
    #[serde(rename = "ROW_TYPE")]
    csv_row_identifier: String,
    #[serde(rename = "FILE_TYPE")]
    category: String,
    #[serde(rename = "FILE_SUBTYPE")]
    report_type: String,
    #[serde(rename = "FILE_DESCRIPTOR")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENTDATE")]
    #[serde(deserialize_with="deserialize_sydney_datetime_to_utc")]
    settlement_date: DateTime<Utc>,
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

impl fmt::Display for InterconnectorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InterconnectorData: row_type: {}, file_type: {}, file_subtype: {}, file_descriptor: {}, settlement_date: {:?}, run_no: {:?}, interconnector_id: {:?}, period_id: {:?}, metered_mw_flow: {:?}, mw_flow: {:?}, mw_losses: {:?}, last_changed: {:?}",
            self.csv_row_identifier, self.category, self.report_type, self.report_type_int,
            self.settlement_date, self.run_no, self.interconnector_id, self.period_id,
            self.metered_mw_flow, self.mw_flow, self.mw_losses, self.last_changed)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PriceData {
    #[serde(rename = "ROW_TYPE")]
    csv_row_identifier: String,
    #[serde(rename = "FILE_TYPE")]
    category: String,
    #[serde(rename = "FILE_SUBTYPE")]
    report_type: String,
    #[serde(rename = "FILE_DESCRIPTOR")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENT_DATE")]
    #[serde(deserialize_with="deserialize_sydney_datetime_to_utc")]
    settlement_date: DateTime<Utc>,
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
    #[serde(deserialize_with="deserialize_sydney_datetime_to_utc")]
    last_changed: DateTime<Utc>,
    #[serde(rename = "ROP")]
    rop: Option<f64>,
    #[serde(rename = "RAISE6SEC_RRP")]
    raise_6_sec_rrp: Option<f64>,
    #[serde(rename = "RAISE6SEC_ROP")]
    raise_6_sec_rop: Option<f64>,
    #[serde(rename = "RAISE60SEC_RRP")]
    raise_60_sec_rrp: Option<f64>,
    #[serde(rename = "RAISE60SEC_ROP")]
    raise_60_sec_rop: Option<f64>,
    #[serde(rename = "RAISE5MIN_RRP")]
    raise_5_min_rrp: Option<f64>,
    #[serde(rename = "RAISE5MIN_ROP")]
    raise_5_min_rop: Option<f64>,
    #[serde(rename = "RAISEREG_RRP")]
    raise_reg_rrp: Option<f64>,
    #[serde(rename = "RAISEREG_ROP")]
    raise_reg_rop: Option<f64>,
    #[serde(rename = "LOWER6SEC_RRP")]
    lower_6_sec_rrp: Option<f64>,
    #[serde(rename = "LOWER6SEC_ROP")]
    lower_6_sec_rop: Option<f64>,
    #[serde(rename = "LOWER60SEC_RRP")]
    lower_60_sec_rrp: Option<f64>,
    #[serde(rename = "LOWER60SEC_ROP")]
    lower_60_sec_rop: Option<f64>,
    #[serde(rename = "LOWER5MIN_RRP")]
    lower_5_min_rrp: Option<f64>,
    #[serde(rename = "LOWER5MIN_ROP")]
    lower_5_min_rop: Option<f64>,
    #[serde(rename = "LOWERREG_RRP")]
    lower_reg_rrp: Option<f64>,
    #[serde(rename = "LOWERREG_ROP")]
    lower_reg_rop: Option<f64>,
    #[serde(rename = "RAISE1SEC_RRP")]
    raise_1_sec_rrp: Option<f64>,
    #[serde(rename = "RAISE1SEC_ROP")]
    raise_1_sec_rop: Option<f64>,
    #[serde(rename = "LOWER1SEC_RRP")]
    lower_1_sec_rrp: Option<f64>,
    #[serde(rename = "LOWER1SEC_ROP")]
    lower_1_sec_rop: Option<f64>,
    #[serde(rename = "PRICE_STATUS")]
    price_status: Option<String>,
}

impl fmt::Display for PriceData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PriceData: {{ csv_row_identifier: {}, category: {}, report_type: {}, report_type_int: {}, settlement_date: {:?}, run_no: {:?}, region_id: {:?}, period_id: {:?}, rrp: {:?}, eep: {:?}, invalid_flag: {:?}, last_changed: {:?}, rop: {:?}, raise_6_sec_rrp: {:?}, raise_6_sec_rop: {:?}, raise_60_sec_rrp: {:?}, raise_60_sec_rop: {:?}, raise_5_min_rrp: {:?}, raise_5_min_rop: {:?}, raise_reg_rrp: {:?}, raise_reg_rop: {:?}, lower_6_sec_rrp: {:?}, lower_6_sec_rop: {:?}, lower_60_sec_rrp: {:?}, lower_60_sec_rop: {:?}, lower_5_min_rrp: {:?}, lower_5_min_rop: {:?}, lower_reg_rrp: {:?}, lower_reg_rop: {:?}, raise_1_sec_rrp: {:?}, raise_1_sec_rop: {:?}, lower_1_sec_rrp: {:?}, lower_1_sec_rop: {:?}, price_status: {:?} }}",
            self.csv_row_identifier,
            self.category,
            self.report_type,
            self.report_type_int,
            self.settlement_date,
            self.run_no,
            self.region_id,
            self.period_id,
            self.rrp,
            self.eep,
            self.invalid_flag,
            self.last_changed,
            self.rop,
            self.raise_6_sec_rrp,
            self.raise_6_sec_rop,
            self.raise_60_sec_rrp,
            self.raise_60_sec_rop,
            self.raise_5_min_rrp,
            self.raise_5_min_rop,
            self.raise_reg_rrp,
            self.raise_reg_rop,
            self.lower_6_sec_rrp,
            self.lower_6_sec_rop,
            self.lower_60_sec_rrp,
            self.lower_60_sec_rop,
            self.lower_5_min_rrp,
            self.lower_5_min_rop,
            self.lower_reg_rrp,
            self.lower_reg_rop,
            self.raise_1_sec_rrp,
            self.raise_1_sec_rop,
            self.lower_1_sec_rrp,
            self.lower_1_sec_rop,
            self.price_status
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
// These are boxed to comply with cargo clippy warning for large enums
pub enum RecordCurrentTradingIs {
    Variant1(Box<InterconnectorData>),
    Variant2(Box<PriceData>),
}

impl fmt::Display for RecordCurrentTradingIs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordCurrentTradingIs::Variant1(interconnector_data) => write!(f, "{}", interconnector_data),
            RecordCurrentTradingIs::Variant2(price_data) => write!(f, "{}", price_data),
        }
    }
}

impl ProcessRecord<RecordCurrentTradingIs> for InterconnectorData {
    fn process(line: &str) -> Result<RecordCurrentTradingIs, Box<dyn Error>> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let record = rdr
            .deserialize::<InterconnectorData>()
            .next()
            .ok_or("Error attempting to deserialize InterconnectorData")??;
        Ok(RecordCurrentTradingIs::Variant1(Box::new(record)))
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
            .ok_or("Error attempting to deserialize PriceData")??;
        Ok(RecordCurrentTradingIs::Variant2(Box::new(record)))
    }
}

pub fn process_file_current_trading_is(
    contents: String,
) -> Result<Vec<RecordCurrentTradingIs>, Box<dyn Error>> {
    let byte_data = contents.as_bytes();
    let reader = BufReader::new(byte_data);
    let mut records: Vec<RecordCurrentTradingIs> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.starts_with("I,TRADING,INTERCONNECTORRES") || line.starts_with("I,TRADING,PRICE") {
            println!("Information Row (header): {:?}", &line);
        }
        match line.chars().next() {
            Some('C') | Some('I') => continue,
            Some('D') => {
                if line.starts_with("D,TRADING,INTERCONNECTORRES") {
                    records.push(InterconnectorData::process(&line)?);
                } else if line.starts_with("D,TRADING,PRICE") {
                    records.push(PriceData::process(&line)?);
                } else {
                    println!("Parser Error: RecordCurrentTradingIs\n line: {:?}", &line);
                    return Err("Unknown record type for RecordCurrentTradingIs Variants".into());
                }
            }
            _ => return Err("Invalid line format".into()),
        }
    }

    Ok(records)
}
