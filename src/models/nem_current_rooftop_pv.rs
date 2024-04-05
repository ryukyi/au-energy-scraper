//! # NEMWEB Trading IS Reports Processor
//!
//! Processes "/Reports/Current/TradingIS_Reports/" from NEMWEB extracting trading data.
//!
//! ## Data Source
//!
//! <https://nemweb.com.au/Reports/Current/TradingIS_Reports/>.
//!
//! Structs and functions are to match below. Note multiple headers in the file.
//! 
//! ./au-energy-scraper/src/fixtures/PUBLIC_TRADINGIS_202403031335_0000000412683134.CSV
//! 
//! ```text title="TradingIS csv"
//! C,NEMP.WORLD,TRADINGIS,AEMO,PUBLIC,2024/03/03,13:30:11,0000000412683134,TRADINGIS,0000000412683133
//! I,TRADING,INTERCONNECTORRES,2,SETTLEMENTDATE,RUNNO,INTERCONNECTORID,PERIODID,METEREDMWFLOW,MWFLOW,MWLOSSES,LASTCHANGED
//! D,TRADING,INTERCONNECTORRES,2,"2024/03/03 13:35:00",1,N-Q-MNSP1,163,36.2,17,1.36,"2024/03/03 13:30:04"
//! D,TRADING,INTERCONNECTORRES,2,"2024/03/03 13:35:00",1,NSW1-QLD1,163,587,464.75,30.31,"2024/03/03 13:30:04"
//! D,TRADING,INTERCONNECTORRES,2,"2024/03/03 13:35:00",1,T-V-MNSP1,163,-407.8,-441,21.94,"2024/03/03 13:30:04"
//! D,TRADING,INTERCONNECTORRES,2,"2024/03/03 13:35:00",1,V-S-MNSP1,163,156,156.22,15.93,"2024/03/03 13:30:04"
//! D,TRADING,INTERCONNECTORRES,2,"2024/03/03 13:35:00",1,V-SA,163,-224.44,-202.19,4.53,"2024/03/03 13:30:04"
//! D,TRADING,INTERCONNECTORRES,2,"2024/03/03 13:35:00",1,VIC1-NSW1,163,-31.1,-392.37,-23.81,"2024/03/03 13:30:04"
//! I,TRADING,PRICE,3,SETTLEMENTDATE,RUNNO,REGIONID,PERIODID,RRP,EEP,INVALIDFLAG,LASTCHANGED,ROP,RAISE6SECRRP,RAISE6SECROP,RAISE60SECRRP,RAISE60SECROP,RAISE5MINRRP,RAISE5MINROP,RAISEREGRRP,RAISEREGROP,LOWER6SECRRP,LOWER6SECROP,LOWER60SECRRP,LOWER60SECROP,LOWER5MINRRP,LOWER5MINROP,LOWERREGRRP,LOWERREGROP,RAISE1SECRRP,RAISE1SECROP,LOWER1SECRRP,LOWER1SECROP,PRICE_STATUS
//! D,TRADING,PRICE,3,"2024/03/03 13:35:00",1,SA1,163,-63.45,0,0,"2024/03/03 13:30:04",-63.45,0,0,0,0,0,0,0.91,0.91,1.84,1.84,4.78,4.78,0.39,0.39,3.76,3.76,0,0,0,0,FIRM
//! D,TRADING,PRICE,3,"2024/03/03 13:35:00",1,NSW1,163,77.06,0,0,"2024/03/03 13:30:04",77.06,0,0,0,0,0,0,0.91,0.91,1.84,1.84,4.78,4.78,0.39,0.39,3.76,3.76,0,0,0,0,FIRM
//! D,TRADING,PRICE,3,"2024/03/03 13:35:00",1,QLD1,163,86,0,0,"2024/03/03 13:30:04",86,0,0,0,0,0,0,0.91,0.91,1.84,1.84,4.78,4.78,0.39,0.39,3.76,3.76,0,0,0,0,FIRM
//! D,TRADING,PRICE,3,"2024/03/03 13:35:00",1,TAS1,163,-40.01,0,0,"2024/03/03 13:30:04",-40.01,0.38,0.38,0.38,0.38,0.38,0.38,6.25,6.25,1.84,1.84,4.78,4.78,0,0,105.27,105.27,0,0,0,0,FIRM
//! D,TRADING,PRICE,3,"2024/03/03 13:35:00",1,VIC1,163,-67.1,0,0,"2024/03/03 13:30:04",-67.1,0,0,0,0,0,0,0.91,0.91,1.84,1.84,4.78,4.78,0.39,0.39,3.76,3.76,0,0,0,0,FIRM
//! C,"END OF REPORT",15
//! ```


use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fmt;

use crate::common::processor::RecordTypeStartsWith;
use crate::time::datetimezone_conversion::deserialize_sydney_datetime_to_utc;

// Updated struct to represent the Data row (D row)
#[derive(Debug, Deserialize)]
pub struct RooftopPvActual {
    // The first three fields are constant and represent metadata about the row
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "D"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "INTERVAL_DATETIMEZONE")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
    interval_datetime: DateTime<Utc>,
    #[serde(rename = "REGIONID")]
    regionid: String,
    #[serde(rename = "POWER")]
    power: Option<f64>,
    #[serde(rename = "QI")]
    qi: Option<f64>,
    #[serde(rename = "TYPE")]
    type_: String,
    #[serde(rename = "LASTCHANGED")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
    lastchanged: DateTime<Utc>,
}

impl fmt::Display for RooftopPvActual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RooftopPvActualData: {{ csv_row_identifier: {:?}, category: {:?}, report_type: {:?}, report_type_int: {:?}, interval_datetime: {:?}, regionid: {:?}, power: {:?}, qi: {:?}, type_: {:?}, lastchanged: {:?} }}",
            self.csv_row_identifier,
            self.category,
            self.report_type,
            self.report_type_int,
            self.interval_datetime,
            self.regionid,
            self.power,
            self.qi,
            self.type_,
            self.lastchanged
        )
    }
}

impl RecordTypeStartsWith for RooftopPvActual {
    fn matches(line: &str) -> bool {
        line.starts_with("D,ROOFTOP,ACTUAL,2")
    }
}

#[derive(Debug, Deserialize)]
pub struct RooftopPvForecast {
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "D"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "VERSION_DATETIME")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
    version_datetime: DateTime<Utc>,
    #[serde(rename = "REGIONID")]
    regionid: String,
    #[serde(rename = "INTERVAL_DATETIME")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
    interval_datetime: DateTime<Utc>,
    #[serde(rename = "POWERMEAN")]
    power_mean: f64,
    #[serde(rename = "POWERPOE50")]
    power_poe50: f64,
    #[serde(rename = "POWERPOELOW")]
    power_poelow: f64,
    #[serde(rename = "POWERPOEHIGH")]
    power_poehigh: f64,
    #[serde(rename = "LASTCHANGED")]
    #[serde(deserialize_with = "deserialize_sydney_datetime_to_utc")]
    lastchanged: DateTime<Utc>,
}

impl fmt::Display for RooftopPvForecast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RooftopPvForecastData: {{ csv_row_identifier: {:?}, category: {:?}, report_type: {:?}, report_type_int: {:?}, version_datetime: {:?}, regionid: {:?}, interval_datetime: {:?}, power_mean: {:?}, power_poe50: {:?}, power_poelow: {:?}, power_poehigh: {:?}, lastchanged: {:?} }}",
            self.csv_row_identifier,
            self.category,
            self.report_type,
            self.report_type_int,
            self.version_datetime,
            self.regionid,
            self.interval_datetime,
            self.power_mean,
            self.power_poe50,
            self.power_poelow,
            self.power_poehigh,
            self.lastchanged
        )
    }
}

impl RecordTypeStartsWith for RooftopPvForecast {
    fn matches(line: &str) -> bool {
        line.starts_with("D,ROOFTOP,FORECAST,1")
    }
}