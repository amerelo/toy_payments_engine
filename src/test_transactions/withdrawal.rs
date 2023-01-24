#[cfg(test)]
mod test {
    use crate::data_types::*;
    use rust_decimal_macros::dec;

    #[test]
    fn ok_withdrawal_0() {
        let mut data = Data::new();
        let tr = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(dec!(2.5)),
        };
        data.execute_transaction(tr);

        let tr = Transaction {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(dec!(2.0)),
        };

        data.execute_transaction(tr);

        let account = data.clients.get(&1).unwrap();

        assert_eq!(
            *account,
            Account {
                available: dec!(0.5),
                held: dec!(0.0),
                total: dec!(0.5),
                locked: false,
            }
        )
    }

    #[test]
    fn ok_withdrawal_2() {
        let mut data = Data::new();
        let tr = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(dec!(2.5)),
        };
        data.execute_transaction(tr);

        let tr = Transaction {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(dec!(2.0)),
        };

        data.execute_transaction(tr);

        let tr = Transaction {
            transaction_type: TransactionType::Withdrawal,
            client: 1,
            tx: 2,
            amount: Some(dec!(2.0)),
        };

        data.execute_transaction(tr);

        let account = data.clients.get(&1).unwrap();

        assert_eq!(
            *account,
            Account {
                available: dec!(0.5),
                held: dec!(0.0),
                total: dec!(0.5),
                locked: false,
            }
        )
    }
}
