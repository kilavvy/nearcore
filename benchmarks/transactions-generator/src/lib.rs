use crate::account::Account;
use near_primitives::transaction::SignedTransaction;
use std::path::Path;
use std::time::Duration;
use tokio::time;
use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::cell::RefCell;

pub mod account;

pub struct TxGenerator {
    accounts: Vec<Account>,
    pacemaker: RefCell<tokio::time::Interval>,
    rnd: RefCell<ThreadRng>,
}

impl TxGenerator {
    pub fn new(accounts_path: &Path, rps: u64)-> anyhow::Result<Self> {
        let accounts = account::accounts_from_dir(accounts_path)?;
        Ok(Self{
            accounts,
            pacemaker: RefCell::new(time::interval(Duration::from_micros(1_000_000/rps))),
            rnd: RefCell::new(rand::thread_rng()),
        })
    }

    pub async fn produce(self: &Self) -> SignedTransaction {
        self.pacemaker.borrow_mut().tick().await;
        let id_sender = Uniform::from(0..self.accounts.len()).sample(&mut *self.rnd.borrow_mut());
        let id_recv = loop {
            let candidate = Uniform::from(0..self.accounts.len()).sample(&mut *self.rnd.borrow_mut());
            if candidate != id_sender {
                break candidate;
            }
        };

        let sender = &accounts[id_sender];
        let receiver = &accounts[id_receiver];
        let transaction = SignedTransaction::send_money(
            sender.nonce + 1, 
            sender.id.clone(),
            receiver.id.clone(),
            &sender.as_signer(),
            amount,
            block_hash(),
        );
        // hierwasik
        panic!("not implemented");
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
