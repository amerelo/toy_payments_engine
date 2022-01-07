#[cfg(test)]
mod test {
    use crate::data_types::*;

    #[test]
    fn ok_chargeback_0() {
        let mut data = Data::new();

        let tr = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.5),
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
            transaction_type: TransactionType::Chargeback,
            client: 1,
            tx: 1,
            amount: None,
        };

        data.execute_transaction(tr);

        let account = data.clients.get(&1).unwrap();

        assert_eq!(
            *account,
            Account {
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: true,
            }
        )
    }

    #[test]
    fn ok_chargeback_1() {
        let mut data = Data::new();

        let tr = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 1,
            amount: Some(2.5),
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
            transaction_type: TransactionType::Chargeback,
            client: 1,
            tx: 1,
            amount: None,
        };

        data.execute_transaction(tr);

        let tr = Transaction {
            transaction_type: TransactionType::Deposit,
            client: 1,
            tx: 2,
            amount: Some(2.5),
        };

        data.execute_transaction(tr);

        let account = data.clients.get(&1).unwrap();

        assert_eq!(
            *account,
            Account {
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: true,
            }
        )
    }
}
