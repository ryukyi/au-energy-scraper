use serde::{self, Deserialize};
use std::fmt;

use crate::common::parser_types::{DataRowTrait, InformationRowTrait};

#[derive(Debug, Deserialize)]
pub struct InterconnectorInformationRow {
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "I"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENTDATE")]
    settlement_date: String,
    #[serde(rename = "RUNNO")]
    run_no: i32,
    #[serde(rename = "INTERCONNECTORID")]
    interconnector_id: String,
    #[serde(rename = "PERIODID")]
    period_id: i32,
    #[serde(rename = "METEREDMWFLOW")]
    metered_mw_flow: f64,
    #[serde(rename = "MWFLOW")]
    mw_flow: f64,
    #[serde(rename = "MWLOSSES")]
    mw_losses: f64,
    #[serde(rename = "LASTCHANGED")]
    last_changed: String,
}

impl InformationRowTrait for InterconnectorInformationRow {}

impl fmt::Display for InterconnectorInformationRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InterconnectorInformationRow: {{ csv_row_identifier: {}, category: {}, report_type: {}, report_type_int: {}, settlement_date: {}, run_no: {}, interconnector_id: {}, period_id: {}, metered_mw_flow: {}, mw_flow: {}, mw_losses: {}, last_changed: {} }}",
            self.csv_row_identifier,
            self.category,
            self.report_type,
            self.report_type_int,
            self.settlement_date,
            self.run_no,
            self.interconnector_id,
            self.period_id,
            self.metered_mw_flow,
            self.mw_flow,
            self.mw_losses,
            self.last_changed
        )
    }
}


#[derive(Debug, Deserialize)]
pub struct InterconnectorDataRow {
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "I"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENTDATE")]
    settlement_date: String,
    #[serde(rename = "RUNNO")]
    run_no: i32,
    #[serde(rename = "INTERCONNECTORID")]
    interconnector_id: String,
    #[serde(rename = "PERIODID")]
    period_id: i32,
    #[serde(rename = "METEREDMWFLOW")]
    metered_mw_flow: f64,
    #[serde(rename = "MWFLOW")]
    mw_flow: f64,
    #[serde(rename = "MWLOSSES")]
    mw_losses: f64,
    #[serde(rename = "LASTCHANGED")]
    last_changed: String,
}

impl fmt::Display for InterconnectorDataRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InterconnectorDataRow: {{ csv_row_identifier: {}, category: {}, report_type: {}, report_type_int: {}, settlement_date: {}, run_no: {}, interconnector_id: {}, period_id: {}, metered_mw_flow: {}, mw_flow: {}, mw_losses: {}, last_changed: {} }}",
            self.csv_row_identifier,
            self.category,
            self.report_type,
            self.report_type_int,
            self.settlement_date,
            self.run_no,
            self.interconnector_id,
            self.period_id,
            self.metered_mw_flow,
            self.mw_flow,
            self.mw_losses,
            self.last_changed
        )
    }
}

// Define structs for PRICE information and data rows
#[derive(Debug, Deserialize)]
pub struct PriceInformationRow {
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "I"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENTDATE")]
    settlement_date: String,
    #[serde(rename = "RUNNO")]
    run_no: String,
    #[serde(rename = "REGIONID")]
    region_id: String,
    #[serde(rename = "PERIODID")]
    period_id: String,
    #[serde(rename = "RRP")]
    rrp: String,
    #[serde(rename = "EEP")]
    eep: String,
    #[serde(rename = "INVALIDFLAG")]
    invalid_flag: String,
    #[serde(rename = "LASTCHANGED")]
    last_changed: String,
    #[serde(rename = "ROP")]
    rop: String,
    #[serde(rename = "RAISE6SECRRP")]
    raise_6_sec_rrp: String,
    #[serde(rename = "RAISE6SECROP")]
    raise_6_sec_rop: String,
    #[serde(rename = "RAISE60SECRRP")]
    raise_60_sec_rrp: String,
    #[serde(rename = "RAISE60SECROP")]
    raise_60_sec_rop: String,
    #[serde(rename = "RAISE5MINRRP")]
    raise_5_min_rrp: String,
    #[serde(rename = "RAISE5MINROP")]
    raise_5_min_rop: String,
    #[serde(rename = "RAISEREGRRP")]
    raise_reg_rrp: String,
    #[serde(rename = "RAISEREGROP")]
    raise_reg_rop: String,
    #[serde(rename = "LOWER6SECRRP")]
    lower_6_sec_rrp: String,
    #[serde(rename = "LOWER6SECROP")]
    lower_6_sec_rop: String,
    #[serde(rename = "LOWER60SECRRP")]
    lower_60_sec_rrp: String,
    #[serde(rename = "LOWER60SECROP")]
    lower_60_sec_rop: String,
    #[serde(rename = "LOWER5MINRRP")]
    lower_5_min_rrp: String,
    #[serde(rename = "LOWER5MINROP")]
    lower_5_min_rop: String,
    #[serde(rename = "LOWERREGRRP")]
    lower_reg_rrp: String,
    #[serde(rename = "LOWERREGROP")]
    lower_reg_rop: String,
    #[serde(rename = "RAISE1SECRRP")]
    raise_1_sec_rrp: String,
    #[serde(rename = "RAISE1SECROP")]
    raise_1_sec_rop: String,
    #[serde(rename = "LOWER1SECRRP")]
    lower_1_sec_rrp: String,
    #[serde(rename = "LOWER1SECROP")]
    lower_1_sec_rop: String,
    #[serde(rename = "PRICE_STATUS")]
    price_status: String,
}

impl fmt::Display for PriceInformationRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PriceIsCurrentInformationRow: {{ csv_row_identifier: {}, category: {}, report_type: {}, report_type_int: {}, settlement_date: {}, run_no: {}, region_id: {}, period_id: {}, rrp: {}, eep: {}, invalid_flag: {}, last_changed: {}, rop: {}, raise_6_sec_rrp: {}, raise_6_sec_rop: {}, raise_60_sec_rrp: {}, raise_60_sec_rop: {}, raise_5_min_rrp: {}, raise_5_min_rop: {}, raise_reg_rrp: {}, raise_reg_rop: {}, lower_6_sec_rrp: {}, lower_6_sec_rop: {}, lower_60_sec_rrp: {}, lower_60_sec_rop: {}, lower_5_min_rrp: {}, lower_5_min_rop: {}, lower_reg_rrp: {}, lower_reg_rop: {}, raise_1_sec_rrp: {}, raise_1_sec_rop: {}, lower_1_sec_rrp: {}, lower_1_sec_rop: {}, price_status: {} }}",
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
pub struct PriceIsCurrentDataRow {
    #[serde(rename = "CSVROWIDENTIFIER")]
    csv_row_identifier: String, // "D"
    #[serde(rename = "CATEGORY")]
    category: String,
    #[serde(rename = "REPORT_TYPE")]
    report_type: String,
    #[serde(rename = "REPORT_TYPE_INT")]
    report_type_int: String,
    #[serde(rename = "SETTLEMENTDATE")]
    settlement_date: String,
    #[serde(rename = "RUNNO")]
    run_no: i32,
    #[serde(rename = "REGIONID")]
    region_id: String,
    #[serde(rename = "PERIODID")]
    period_id: i32,
    #[serde(rename = "RRP")]
    rrp: f64,
    #[serde(rename = "EEP")]
    eep: f64,
    #[serde(rename = "INVALIDFLAG")]
    invalid_flag: i32,
    #[serde(rename = "LASTCHANGED")]
    last_changed: String,
    #[serde(rename = "ROP")]
    rop: f64,
    #[serde(rename = "RAISE6SECRRP")]
    raise_6_sec_rrp: f64,
    #[serde(rename = "RAISE6SECROP")]
    raise_6_sec_rop: f64,
    #[serde(rename = "RAISE60SECRRP")]
    raise_60_sec_rrp: f64,
    #[serde(rename = "RAISE60SECROP")]
    raise_60_sec_rop: f64,
    #[serde(rename = "RAISE5MINRRP")]
    raise_5_min_rrp: f64,
    #[serde(rename = "RAISE5MINROP")]
    raise_5_min_rop: f64,
    #[serde(rename = "RAISEREGRRP")]
    raise_reg_rrp: f64,
    #[serde(rename = "RAISEREGROP")]
    raise_reg_rop: f64,
    #[serde(rename = "LOWER6SECRRP")]
    lower_6_sec_rrp: f64,
    #[serde(rename = "LOWER6SECROP")]
    lower_6_sec_rop: f64,
    #[serde(rename = "LOWER60SECRRP")]
    lower_60_sec_rrp: f64,
    #[serde(rename = "LOWER60SECROP")]
    lower_60_sec_rop: f64,
    #[serde(rename = "LOWER5MINRRP")]
    lower_5_min_rrp: f64,
    #[serde(rename = "LOWER5MINROP")]
    lower_5_min_rop: f64,
    #[serde(rename = "LOWERREGRRP")]
    lower_reg_rrp: f64,
    #[serde(rename = "LOWERREGROP")]
    lower_reg_rop: f64,
    #[serde(rename = "RAISE1SECRRP")]
    raise_1_sec_rrp: f64,
    #[serde(rename = "RAISE1SECROP")]
    raise_1_sec_rop: f64,
    #[serde(rename = "LOWER1SECRRP")]
    lower_1_sec_rrp: f64,
    #[serde(rename = "LOWER1SECROP")]
    lower_1_sec_rop: f64,
    #[serde(rename = "PRICE_STATUS")]
    price_status: String,
}

impl DataRowTrait for PriceIsCurrentDataRow {}

impl fmt::Display for PriceIsCurrentDataRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PriceIsCurrentDataRow: {{ csv_row_identifier: {}, category: {}, report_type: {}, report_type_int: {}, settlement_date: {}, run_no: {}, region_id: {}, period_id: {}, rrp: {}, eep: {}, invalid_flag: {}, last_changed: {}, rop: {}, raise_6_sec_rrp: {}, raise_6_sec_rop: {}, raise_60_sec_rrp: {}, raise_60_sec_rop: {}, raise_5_min_rrp: {}, raise_5_min_rop: {}, raise_reg_rrp: {}, raise_reg_rop: {}, lower_6_sec_rrp: {}, lower_6_sec_rop: {}, lower_60_sec_rrp: {}, lower_60_sec_rop: {}, lower_5_min_rrp: {}, lower_5_min_rop: {}, lower_reg_rrp: {}, lower_reg_rop: {}, raise_1_sec_rrp: {}, raise_1_sec_rop: {}, lower_1_sec_rrp: {}, lower_1_sec_rop: {}, price_status: {} }}",
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