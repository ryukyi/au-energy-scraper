use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use std::str;

trait CsvParsable: for<'de> serde::Deserialize<'de> {
    fn parse_csv<R: std::io::Read>(reader: R) -> Result<Vec<Self>, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut rdr = csv::Reader::from_reader(reader);
        let mut results = Vec::new();
        for result in rdr.deserialize() {
            let record: Self = result?;
            results.push(record);
        }
        Ok(results)
    }
}


// Enum to represent the row types
enum RowType {
    Control,
    Information,
    Data,
}

type Header = String;

// Information will serve as headers
#[derive(Debug, Deserialize)]
struct InformationRow {
    // first 4 columns not needed
    interval_datetime: Header,
    regionid: Header,
    power: Header,
    qi: Header,
    type_: Header,
    lastchanged: Header,
}

// Struct to represent the Data rows
#[derive(Debug, Deserialize)]
struct DataRow {
    interval_datetime: String,
    regionid: String,
    power: f64,
    qi: u32,
    type_: String,
    lastchanged: String,
}

trait ParseFromLine: for<'de> Deserialize<'de> {}
impl ParseFromLine for InformationRow {}
impl ParseFromLine for DataRow {}

#[derive(Debug)]
pub struct ParsedData {
    headers: Vec<InformationRow>,
    data: Vec<DataRow>,
}

struct ParsingState {
    parsed_data_sets: Vec<ParsedData>,
    current_headers: Vec<InformationRow>,
    current_data: Vec<DataRow>,
    last_row_type: RowType,
}

impl ParsingState {
    fn new() -> Self {
        ParsingState {
            parsed_data_sets: Vec::new(),
            current_headers: Vec::new(),
            current_data: Vec::new(),
            last_row_type: RowType::Control, // Initial row type
        }
    }

    fn start_new_data_set(&mut self) {
        if !self.current_headers.is_empty() || !self.current_data.is_empty() {
            self.parsed_data_sets.push(ParsedData {
                headers: std::mem::take(&mut self.current_headers),
                data: std::mem::take(&mut self.current_data),
            });
        }
    }

    fn add_header(&mut self, header: InformationRow) {
        self.current_headers.push(header);
    }

    fn add_data_row(&mut self, data_row: DataRow) {
        self.current_data.push(data_row);
    }

    fn finalize(&mut self) {
        if !self.current_headers.is_empty() || !self.current_data.is_empty() {
            let parse_data = ParsedData {
                headers: self.current_headers.drain(..).collect(),
                data: self.current_data.drain(..).collect(),
            };
            self.parsed_data_sets.push(parse_data);
        }
    }
}

fn get_relevant_byte_slice(line: &[u8]) -> &[u8] {
    let mut commas_count = 0;
    let mut last_comma_index = 0;
    // Iterate over bytes to find the index after the 4th comma
    for (index, &byte) in line.iter().enumerate() {
        if byte == b',' {
            commas_count += 1;
            if commas_count == 4 {
                last_comma_index = index + 1; // Move past the 4th comma
                break;
            }
        }
    }
    // Slice the line to skip the first four fields and remove trailing \r\n if present
    let mut end_index = line.len();
    if line.ends_with(b"\r\n") {
        end_index -= 2;
    } else if line.ends_with(b"\n") {
        end_index -= 1;
    }

    &line[last_comma_index..end_index]
}

fn parse_line<T: ParseFromLine>(line: &[u8]) -> Option<T> {
    let relevant_slice = get_relevant_byte_slice(line);
    let relevant_str = String::from_utf8_lossy(relevant_slice);
    serde_json::from_str(&relevant_str).ok()
}

// Accepts generic reader traits so can handle zip crate ZipFile type
pub fn parse_csv_no_dependencies<R: std::io::Read>(reader: R) -> Result<Vec<ParsedData>, Box<dyn Error>> {
    let mut reader = BufReader::new(reader);
    let mut line: Vec<u8> = Vec::new();
    let mut state = ParsingState::new();

    // Read lines as bytes
    while reader.read_until(b'\n', &mut line)? > 0 {
        if let Some(&first_byte) = line.first() {
            match first_byte {
                b'C' => {
                    // Handle Control or Comment row
                    let line_str = String::from_utf8_lossy(&line);
                    println!("Control row: {:?}", line_str);
                }
                b'I' => {
                    // Handle Information or Data row
                    let relevant_slice = get_relevant_byte_slice(&line);
                    // Convert the relevant slice to a string
                    let relevant_str = String::from_utf8_lossy(relevant_slice);
                    let fields: Vec<&str> = relevant_str.split(',').collect();
                    let information_row = InformationRow {
                        interval_datetime: fields[0].to_string(),
                        regionid: fields[1].to_string(),
                        power: fields[2].to_string(),
                        qi: fields[3].to_string(),
                        type_: fields[4].to_string(),
                        lastchanged: fields[5].to_string(),
                    };
                    println!("{:?}", information_row);
                    if matches!(state.last_row_type, RowType::Data) {
                        state.start_new_data_set();
                    }
                    // Parse and add to current_headers
                    state.last_row_type = RowType::Information;
                }
                b'D' => {
                    // Handle Information or Data row
                    let relevant_slice = get_relevant_byte_slice(&line);
                    // Convert the relevant slice to a string
                    let relevant_str = String::from_utf8_lossy(relevant_slice);
                    let fields: Vec<&str> = relevant_str.split(',').collect();
                    if fields.len() >= 5 {
                        let data_row = DataRow {
                            interval_datetime: fields[0].to_string(),
                            regionid: fields[1].to_string(),
                            power: fields[2].parse().unwrap_or_default(),
                            qi: fields[3].parse().unwrap_or_default(),
                            type_: fields[4].to_string(),
                            lastchanged: fields[5].parse().unwrap_or_default(),
                        };
                        println!("{:?}", data_row)
                    }
                    state.last_row_type = RowType::Data;
                }
                _ => {}
            }
        }
        line.clear();
    }

    // Finalize to ensure the last data set is added
    state.finalize();

    Ok(state.parsed_data_sets)
}