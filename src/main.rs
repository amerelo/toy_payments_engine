mod csv;
mod data_types;

mod test_transactions;

fn main() {
    env_logger::init();

    if let Err(err) = csv::read_csv() {
        println!("error running example: {}", err);
    }
}
