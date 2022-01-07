This is a simple toy payments engine that reads a series of transactions
from a CSV, updates client accounts, handles disputes and chargebacks, and then outputs the
state of clients accounts as a CSV.


you can run the program with:
cargo run -- transactions.csv > accounts.csv

you can also run the unit-test with 
cargo test