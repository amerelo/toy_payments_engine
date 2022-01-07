#[cfg(test)]
mod test {
    use crate::data_types::*;

    #[test]
    fn ok_deposit_0() {
        let mut data = Data::new();

        let tr = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.5),
        };

        data.execute_transaction(tr);

        let account = data.clients.get(&1).unwrap();

        assert_eq!(
            *account,
            Account {
                available: 2.5,
                held: 0.0,
                total: 2.5,
                locked: false,
            }
        )
    }
}
