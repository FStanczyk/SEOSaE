use std::{error::Error, fs::OpenOptions};
use crate::matching;
use csv;
pub fn process_data(buy_orders: &mut Vec<(f64,u32)>, sell_orders: &mut Vec<(f64,u32)>) -> Result<(), Box<dyn Error>>  {

    let mut rdr = csv::Reader::from_path(crate::PATH_TO_READ_FILE)?;
    for result in rdr.records() {
        let record = result?;

        let is_buy_order:bool = record[0].parse()?;
        let price:f64 = record[1].parse()?;
        let amount:u32 = record[2].parse()?;

        let mut new_order = (price,amount);
        if is_buy_order {
            buy_orders.push(new_order);
            matching::matching(sell_orders, &mut new_order);
        }else{
            sell_orders.push(new_order);
            matching::matching(buy_orders, &mut new_order);
        }
    }
    Ok(())
}

pub fn write_log(vector: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).open("logs.csv")?;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);
    wtr.write_record(vector)?;
    wtr.flush()?;
    Ok(())
}


pub fn clear_logs() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("logs.csv")?;
    wtr.write_record(&["Action", "sell/buy", "Price", "Amount","matched with","price","amount"])?;
    wtr.flush()?;
    Ok(())
}