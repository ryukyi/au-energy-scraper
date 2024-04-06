//! # NEMWEB Rooftop PV Actual Data Processor
//!
//! Processes "https://nemweb.com.au/Reports/Current/ROOFTOP_PV/ACTUAL/" from NEMWEB, extracting rooftop solar generation data.
//!
//! ## Data Source
//!
//! <https://nemweb.com.au/Reports/Current/ROOFTOP_PV/ACTUAL/>.
//!
//! Structs and functions are to match:
//!
//! ./au-energy-scraper/src/fixtures/PUBLIC_ROOFTOP_PV_ACTUAL_MEASUREMENT_20240303200000_0000000412707330.csv
//!
//! ```text title="rooftop actual csv"
//! C,NEMP.WORLD,ROOFTOP_PV_ACTUAL_MEASUREMENT,AEMO,PUBLIC,2024/03/03,20:00:03,0000000412707330,DEMAND,0000000412707330
//! I,ROOFTOP,ACTUAL,2,INTERVAL_DATETIME,REGIONID,POWER,QI,TYPE,LASTCHANGED
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",NSW1,0,1,MEASUREMENT,"2024/03/03 19:49:13"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLD1,0,1,MEASUREMENT,"2024/03/03 19:49:13"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLDC,0,1,MEASUREMENT,"2024/03/03 19:49:13"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLDN,0,1,MEASUREMENT,"2024/03/03 19:49:14"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",QLDS,0,1,MEASUREMENT,"2024/03/03 19:49:14"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",SA1,6.617,1,MEASUREMENT,"2024/03/03 19:49:14"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",TAS1,0,1,MEASUREMENT,"2024/03/03 19:49:14"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",TASN,0,1,MEASUREMENT,"2024/03/03 19:49:14"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",TASS,0,1,MEASUREMENT,"2024/03/03 19:49:14"
//! D,ROOFTOP,ACTUAL,2,"2024/03/03 19:30:00",VIC1,0,1,MEASUREMENT,"2024/03/03 19:49:14"
//! C,"END OF REPORT",13
//! ```

use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;

use crate::common::processor::RecordTypeStartsWith;
use crate::time::datetimezone_conversion::deserialize_sydney_datetime_to_utc;
#[derive(Debug, Deserialize, Default)]
pub struct Interconnector {
    #[serde(rename = "ROW_TYPE")]
    csv_row_identifier: String,
    #[serde(rename = "FILE_TYPE")]
    category: String,
    #[serde(rename = "FILE_SUBTYPE")]
    report_type: String,
    #[serde(rename = "FILE_DESCRIPTOR")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENTDATE")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
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

impl fmt::Display for Interconnector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InterconnectorData: row_type: {}, file_type: {}, file_subtype: {}, file_descriptor: {}, settlement_date: {:?}, run_no: {:?}, interconnector_id: {:?}, period_id: {:?}, metered_mw_flow: {:?}, mw_flow: {:?}, mw_losses: {:?}, last_changed: {:?}",
            self.csv_row_identifier, self.category, self.report_type, self.report_type_int,
            self.settlement_date, self.run_no, self.interconnector_id, self.period_id,
            self.metered_mw_flow, self.mw_flow, self.mw_losses, self.last_changed)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Price {
    #[serde(rename = "ROW_TYPE")]
    csv_row_identifier: String,
    #[serde(rename = "FILE_TYPE")]
    category: String,
    #[serde(rename = "FILE_SUBTYPE")]
    report_type: String,
    #[serde(rename = "FILE_DESCRIPTOR")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENT_DATE")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
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
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
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

impl fmt::Display for Price {
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

impl RecordTypeStartsWith for Interconnector {
    fn matches(line: &str) -> bool {
        line.starts_with("D,TRADING,INTERCONNECTORRES,2")
    }
}

impl RecordTypeStartsWith for Price {
    fn matches(line: &str) -> bool {
        line.starts_with("D,TRADING,PRICE,3")
    }
}
