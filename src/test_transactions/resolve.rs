#[cfg(test)]
mod test {
    use crate::data_types::*;
    use rust_decimal_macros::dec;

    #[test]
    fn ok_resolve_0() {
        let mut data = Data::new();

        let tr = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(dec!(2.5)),
        };

        data.execute_transaction(tr);

        let tr = Transaction {
            transaction_type: TransactionType::Dispute,
            client: 1,
            tx: 1,
            amount: None,
        };

        data.execute_transaction(tr);

        let tr = Transaction {
            transaction_type: TransactionType::Resolve,
            client: 1,
            tx: 1,
            amount: None,
        };

        data.execute_transaction(tr);

        let account = data.clients.get(&1).unwrap();

        assert_eq!(
            *account,
            Account {
                available: dec!(2.5),
                held: dec!(0.0),
                total: dec!(2.5),
                locked: false,
            }
        )
    }
}
