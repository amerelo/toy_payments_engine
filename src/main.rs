mod csv;
mod data_types;

fn main() {
    if let Err(err) = csv::read_csv() {
        println!("error running example: {}", err);
    }
}
