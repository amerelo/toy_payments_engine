use std::error::Error;
use std::process;

use serde::Deserialize;

use std::env;


#[derive(Debug, Deserialize)]
struct Output {
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename(deserialize = "type"))]
    transaction_type: TransactionType,
    client: u16,
    tx: u32,
    amount: f64,
}

fn example() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    //from_path csv/csv1.csv
    // let mut rdr = csv::Reader::from_path(&args[1]).unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(&args[1])
        .unwrap();

    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

pub fn read_csv() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}