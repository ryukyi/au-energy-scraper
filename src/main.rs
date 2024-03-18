use std::fs;
use std::error::Error;
use std::str::FromStr;

// Trait for parsing CSV data into a struct
// Adjusted CsvParsable trait without Sized bound
trait CsvParsable {
    fn parse_from_csv(fields: &[String]) -> Result<Self, Box<dyn Error>> where Self: Sized;
}

// Struct representing the interconnector data row
#[derive(Debug)]
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

impl CsvParsable for InterconnectorData {
    fn parse_from_csv(fields: &[String]) -> Result<Self, Box<dyn Error>> {
        Ok(InterconnectorData {
            row_type: fields[0].to_string(),
            file_type: fields[1].to_string(),
            file_subtype: fields[2].to_string(),
            file_descriptor: fields[3].to_string(),
            settlement_date: fields[4].to_string(),
            run_no: u32::from_str(&fields[5])?,
            interconnector_id: fields[6].to_string(),
            period_id: u32::from_str(&fields[7])?,
            metered_mw_flow: f64::from_str(&fields[8])?,
            mw_flow: f64::from_str(&fields[9])?,
            mw_losses: f64::from_str(&fields[10])?,
            last_changed: fields[11].to_string(),
        })
    }
}

// Struct representing the price data row
#[derive(Debug)]
struct PriceData {
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

impl CsvParsable for PriceData {
    fn parse_from_csv(fields: &[String]) -> Result<Self, Box<dyn Error>> {
        Ok(PriceData {
            row_type: fields[0].to_string(),
            file_type: fields[1].to_string(),
            file_subtype: fields[2].to_string(),
            file_descriptor: fields[3].to_string(),
            settlement_date: fields[4].to_string(),
            run_no: u32::from_str(&fields[5])?,
            region_id: fields[6].to_string(),
            period_id: u32::from_str(&fields[7])?,
            rrp: f64::from_str(&fields[8])?,
            eep: f64::from_str(&fields[9])?,
            invalid_flag: u32::from_str(&fields[10])?,
            last_changed: fields[11].to_string(),
            rop: f64::from_str(&fields[12])?,
            raise6sec_rrp: f64::from_str(&fields[13])?,
            raise6sec_rop: f64::from_str(&fields[14])?,
            raise60sec_rrp: f64::from_str(&fields[15])?,
            raise60sec_rop: f64::from_str(&fields[16])?,
            raise5min_rrp: f64::from_str(&fields[17])?,
            raise5min_rop: f64::from_str(&fields[18])?,
            raisereg_rrp: f64::from_str(&fields[19])?,
            raisereg_rop: f64::from_str(&fields[20])?,
            lower6sec_rrp: f64::from_str(&fields[21])?,
            lower6sec_rop: f64::from_str(&fields[22])?,
            lower60sec_rrp: f64::from_str(&fields[23])?,
            lower60sec_rop: f64::from_str(&fields[24])?,
            lower5min_rrp: f64::from_str(&fields[25])?,
            lower5min_rop: f64::from_str(&fields[26])?,
            lowerreg_rrp: f64::from_str(&fields[27])?,
            lowerreg_rop: f64::from_str(&fields[28])?,
            raise1sec_rrp: f64::from_str(&fields[29])?,
            raise1sec_rop: f64::from_str(&fields[30])?,
            lower1sec_rrp: f64::from_str(&fields[31])?,
            lower1sec_rop: f64::from_str(&fields[32])?,
            price_status: fields[33].to_string(),
        })
    }
}

// Adjusted CsvParsable trait without Sized bound
enum CsvRecord {
    Interconnector(InterconnectorData),
    Price(PriceData),
}

// CSV parser struct
struct CsvParser {
    records: Vec<CsvRecord>,
}

impl CsvParser {
    fn new() -> Self {
        CsvParser {
            records: Vec::new(),
        }
    }

    fn add_record(&mut self, record: CsvRecord) {
        self.records.push(record);
    }
}

enum RowType {
    Control,
    Header,
    Data,
}

impl CsvParser {
    fn parse_csv_content(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        for line in content.lines() {
            let row_type = Self::determine_row_type(line)?;
            match row_type {
                RowType::Control => continue, // Skip control rows
                RowType::Header => {
                    // TODO: include logic to check if its a new Header after adding Data
                },
                RowType::Data => {
                    let fields: Vec<String> = line.split(',').map(|field| field.to_string()).collect();
                    match fields[2].as_str() {
                        "INTERCONNECTORRES" => {
                            let data = InterconnectorData::parse_from_csv(&fields)?;
                            self.add_record(CsvRecord::Interconnector(data));
                        },
                        "PRICE" => {
                            let data = PriceData::parse_from_csv( &fields)?;
                            self.add_record(CsvRecord::Price(data));
                        },
                        _ => {} // Handle other cases or ignore
                    }
                    }
            }
        }

        Ok(())
    }


    // Helper method to determine the row type based on the first character
    fn determine_row_type(line: &str) -> Result<RowType, Box<dyn std::error::Error>> {
        match line.chars().next() {
            Some('C') => Ok(RowType::Control),
            Some('I') => Ok(RowType::Header),
            Some('D') => Ok(RowType::Data),
            _ => Err("Unknown row type".into()),
        }
    }
}



fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "src/fixtures/PUBLIC_TRADINGIS_202403031335_0000000412683134.CSV";
    let content = fs::read_to_string(file_path)?;

    let mut csv_parser = CsvParser::new();
    csv_parser.parse_csv_content(&content)?;

    for record in csv_parser.records {
        match record {
            CsvRecord::Interconnector(data) => println!("Interconnector Data: {:?}", data),
            CsvRecord::Price(data) => println!("Price Data: {:?}", data),
        }
    }

    Ok(())
}
