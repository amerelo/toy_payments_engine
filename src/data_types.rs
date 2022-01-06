use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, HashMap},
    hash::{Hash, Hasher},
    error::Error,
};

#[derive(Debug)]
pub struct Data {
    clients: HashMap<u16, Account>,
    txs: HashSet<Transaction>,
    disputed_tr: HashSet<u32>,
}

#[derive(Debug)]
pub enum Successful {
    True,
    False
}

impl Data {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            txs: HashSet::new(),
            disputed_tr: HashSet::new(),
        }
    }

    pub fn write_record<W>(&mut self, wtr: &mut csv::Writer<W>) -> Result<(), Box<dyn Error>>
    where
        W: std::io::Write,
    {
        for (client_id, account) in self.clients.iter() {
            account.write_record(*client_id, wtr)?;
        }

        Ok(())
    }

    fn create_new_client(&mut self, client_id: u16) {
        self.clients.insert(client_id, Account::new());
    }

    pub fn make_transaction(&mut self, tr: &Transaction) -> Successful {

        // transaction Checks
        // - check if client exist and create one if not
        // - check if client account is locked
        match self.clients.get(&tr.client) {
            Some(account) => {
                if account.is_lock() {
                    // TODO: log erro
                    return Successful::False
                }
            }
            None => {
                self.create_new_client(tr.client);
            }
        }

        let account = if let Some(account) = self.clients.get_mut(&tr.client) {
            account
        } else {
            // TODO: log
            return Successful::False
        };

        match tr.transaction_type {
            TransactionType::Deposit => {
                account.deposit(tr.amount)
            }
            TransactionType::Withdrawal => {
                account.withdrawal(tr.amount)
            }
            TransactionType::Dispute => {
                match self.txs.get(&tr) {
                    Some(disputed_tr) => {
                        // if the dispute is successful add tx to dispute list
                        if let Successful::True = account.dispute(disputed_tr) {
                            self.disputed_tr.insert(tr.tx);

                            Successful::True
                        } else {
                            // TODO: LOG ERRRO
                            Successful::False
                        }
                    }
                    None => {
                        // TODO: log error
                        Successful::False
                    }
                }
            }
            TransactionType::Resolve => {
                match self.txs.get(&tr) {
                    Some(disputed_tr) => {
                        // if the resolve is successful remove tx from dispute list
                        if let Successful::True = account.resolve(disputed_tr) {
                            self.disputed_tr.remove(&tr.tx);

                            Successful::True
                        } else {
                            Successful::False
                        }
                    }
                    None => {
                        // TODO: log error
                        Successful::False
                    }
                }
            }
            TransactionType::Chargeback => {
                match self.txs.get(&tr) {
                    Some(disputed_tr) => {
                        self.disputed_tr.insert(tr.tx);

                        // if the chargeback is successful remove tx from dispute list
                        if let Successful::True = account.chargeback(disputed_tr) {
                            self.disputed_tr.remove(&tr.tx);

                            Successful::True
                        } else {
                            Successful::False
                        }
                    }
                    None => {
                        // TODO: log error
                        Successful::False
                    }
                }
            }
        }
    }

    pub fn execute_transaction(&mut self, tr: Transaction) {
        if !self.txs.contains(&tr) {
            if let Successful::True = self.make_transaction(&tr) {
                self.txs.insert(tr);
            }
        }
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

impl Account {
    pub fn new() -> Self {
        Self {
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn write_record<W>(&self, client_id: u16, wtr: &mut csv::Writer<W>) -> Result<(), Box<dyn Error>>
    where
        W: std::io::Write,
    {
        wtr.serialize((
            client_id,
            self.available,
            self.held,
            self.total,
            self.locked,
        ))?;

        Ok(())
    }


    pub fn deposit(&mut self, amount: f64) -> Successful {
        if amount.is_sign_negative() {
            // TODO: log error
            return Successful::False
        }

        self.available += amount;
        self.total += amount;

        Successful::True
    }

    pub fn withdrawal(&mut self, amount: f64) -> Successful {
        if amount.is_sign_negative() {
            // TODO: log error
            return Successful::False
        }

        if self.available < amount {
            // TODO: log error
            return Successful::False
        }

        self.available -= amount;
        self.total -= amount;

        Successful::True
    }

    pub fn dispute(&mut self, disputed_tr: &Transaction) -> Successful {
        match disputed_tr.transaction_type {
            TransactionType::Deposit => {
                self.available -= disputed_tr.amount;
                self.held += disputed_tr.amount;

                Successful::True
            }
            _ => {
                // TODO: log Error
                Successful::False
            }
        }
    }

    pub fn resolve(&mut self, disputed_tr: &Transaction) -> Successful {
        match disputed_tr.transaction_type {
            TransactionType::Deposit => {
                self.available += disputed_tr.amount;
                self.held -= disputed_tr.amount;

                Successful::True
            }
            _ => {
                // TODO: log Error
                Successful::False
            }
        }
    }

    pub fn chargeback(&mut self, disputed_tr: &Transaction) -> Successful {
        match disputed_tr.transaction_type {
            TransactionType::Deposit => {
                self.held -= disputed_tr.amount;
                self.total -= disputed_tr.amount;
                self.locked = true;

                Successful::True
            }
            _ => {
                // TODO: log Error
                Successful::False
            }
        }
    }

    pub fn is_lock(&self) -> bool {
        self.locked
    }
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub transaction_type: TransactionType,
    pub  client: u16,
    pub  tx: u32,
    pub  amount: f64,
}

impl Eq for Transaction {}

impl PartialEq for Transaction {
    fn eq(&self, other: &Transaction) -> bool {
        self.tx == other.tx
    }
}

impl Hash for Transaction {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.tx.hash(state);
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
