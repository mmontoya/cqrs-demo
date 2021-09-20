use cqrs_es::{EventEnvelope, Query, QueryProcessor};
use serde::{Deserialize, Serialize};

use crate::aggregate::BankAccount;
use crate::events::BankAccountEvent;

pub struct SimpleLoggingQueryProcessor {}

impl QueryProcessor<BankAccount> for SimpleLoggingQueryProcessor {
    fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            let payload = serde_json::to_string_pretty(&event.payload).unwrap();
            println!("{}-{}\n{}", aggregate_id, event.sequence, payload);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankAccountQuery {
    account_id: Option<String>,
    balance: f64,
    written_checks: Vec<String>,
}

impl Query<BankAccount> for BankAccountQuery {
    fn update(&mut self, event: &EventEnvelope<BankAccount>) {
        match &event.payload {
            BankAccountEvent::AccountOpened { account_id } => {
                self.account_id = Some(account_id.clone());
            }
            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = *balance;
            }
            BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = *balance;
            }
            BankAccountEvent::CustomerWroteCheck {
                check_number,
                amount: _,
                balance,
            } => {
                self.balance = *balance;
                self.written_checks.push(check_number.clone())
            }
        }
    }
}

impl Default for BankAccountQuery {
    fn default() -> Self {
        BankAccountQuery {
            account_id: None,
            balance: 0_f64,
            written_checks: Default::default(),
        }
    }
}
