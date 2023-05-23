use std::{hint, time};
use rand::{Rng};
use rand::rngs::ThreadRng;

//declare and initialize pieces/parts to be used
#[derive(Debug, Copy, Clone)]
pub struct Account {
    account_id: u32,
    current_bill: i32,
    balance: i32,
    paid_amount: i32,
}

impl Account {
    fn rand(id: u32, rng: &mut ThreadRng) -> Self {
        Self {
            account_id: id,
            current_bill: rng.gen_range(0..100),
            balance: rng.gen_range(0..100),
            paid_amount: rng.gen_range(0..100)
        }
    }
}

struct AccountSource {
    rng: ThreadRng,
    max: u32,
}

impl AccountSource {
    fn new(n: u32, rng: ThreadRng) -> Self {
        Self {
            rng,
            max: n,
        }
    }
}

impl IntoIterator for AccountSource {
    type Item = Account;
    type IntoIter = Box<dyn Iterator<Item = Account>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new((0..self.max)
            .map(move |i| Account::rand(i, &mut self.rng.to_owned())))
    }
}

pub fn benchmark_inner(ops: u32) -> Vec<Account> {
    //initialize psuedo random seed
    let mut rng = rand::thread_rng();

    // black box
    let accounts: Vec<Account> = hint::black_box(AccountSource::new(ops, rng.to_owned()).into_iter()
        .map(|mut account| {
            let payment = account.balance.min(account.current_bill);
            account.paid_amount += payment;
            account.paid_amount >>= 2;
            account.current_bill += account.current_bill - payment + rng.gen_range(0..100);
            account.current_bill >>= 2;
            account.balance += rng.gen_range(0..100);
            hint::black_box(account)
        }).collect());

    hint::black_box(accounts)
}

pub fn benchmark(ops: u32) -> time::Duration {

    let start = std::time::Instant::now();
    let _ = benchmark_inner(ops);
    //grab time
    start.elapsed()
}
