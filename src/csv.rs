use std::error::Error;

use crate::data_types::{Data, Transaction};
use std::env;

pub fn read_csv() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(&args[1])?;

    let mut data = Data::new();

    for result in rdr.deserialize() {
        let tr: Transaction = result?;
        data.execute_transaction(tr)
    }

    write_csv(&mut data)?;

    Ok(())
}

fn write_csv(data: &mut Data) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(&["client", "available", "held", "total", "locked"])?;

    data.write_record(&mut wtr)?;

    wtr.flush()?;
    Ok(())
}
