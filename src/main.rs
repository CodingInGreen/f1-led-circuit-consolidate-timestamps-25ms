use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    designator: String,
    driver_number: i32,
}

#[derive(Debug, Serialize, Clone)]
struct OutputRecord {
    timestamp: String,
    led_num: String,
    driver_number: i32,
}

fn parse_timestamp(timestamp: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
    let datetime = timestamp.parse::<DateTime<Utc>>()?;
    Ok(datetime)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/hott/eng/f1-led-circuit-consolidate-timestamps/track_data_short_sample.csv";
    let output_path = "output.csv";
    
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    
    let mut records: Vec<Record> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    
    if records.is_empty() {
        return Err("No records found in the input file.".into());
    }
    
    let first_timestamp = parse_timestamp(&records[0].date)?;
    let mut anchor_points = HashMap::new();
    let mut current_anchor = first_timestamp;
    
    while current_anchor <= parse_timestamp(&records[records.len() - 1].date)? {
        anchor_points.insert(current_anchor, current_anchor);
        current_anchor = current_anchor + chrono::Duration::milliseconds(25);
    }
    
    let mut output_records: Vec<OutputRecord> = Vec::new();
    
    for record in records {
        let record_timestamp = parse_timestamp(&record.date)?;
        let nearest_anchor = anchor_points.keys()
            .min_by_key(|&&k| (k - record_timestamp).num_milliseconds().abs())
            .unwrap()
            .clone();
        
        let led_num = record.designator.trim_start_matches('U').to_string();
        
        let output_record = OutputRecord {
            timestamp: nearest_anchor.to_rfc3339(),
            led_num,
            driver_number: record.driver_number,
        };
        
        output_records.push(output_record);
    }
    
    let mut wtr = WriterBuilder::new().from_path(output_path)?;
    
    for output_record in output_records {
        wtr.serialize(output_record)?;
    }
    
    wtr.flush()?;
    Ok(())
}
