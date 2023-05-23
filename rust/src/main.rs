use std::{hint, time};
use rand::{Rng, ThreadRng};

//declare and initialize pieces/parts to be used
#[derive(Debug, Copy, Clone)]
struct Account {
    account_id: u32,
    current_bill: i32,
    balance: i32,
    paid_amount: i32,
}

impl Account {
    fn rand(id: u32, rng: &mut ThreadRng) -> Self {
        Self {
            account_id: id,
            current_bill: rng.gen_range(0, 100),
            balance: rng.gen_range(0, 100),
            paid_amount: rng.gen_range(0, 100)
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


fn main() {
    
    //declare oper/iter size and timer parts
    let argv : Vec<String> = std::env::args().collect();
    let opers: u32 = if argv.len() > 1 { str::parse::<u32>(&argv[1]).unwrap() } else {100};
    let iters: u32 = if argv.len() > 2 { str::parse::<u32>(&argv[2]).unwrap() } else {100};

    // thread_rng is lazily created
    // (eg first execution will be slower than following executions on the same thread)
    // let _ = rand::thread_rng();

    let mut min = std::time::Duration::new(9999, 0); 
    let mut max = std::time::Duration::new(0, 0); 
    let mut average = std::time::Duration::new(0, 0);
    let total = std::time::Instant::now();

    //Declare and begin iterations
    println!("Rust performing {} operations over {} iterations.", opers, iters);
    (0..iters).for_each(|_| {
        //start timer
        let diff = hint::black_box(benchmark(opers));
        if diff < min { min = diff };
        if diff > max { max = diff };
        average += diff;
    });

    //print result
    println!("Done! after {:?}", total.elapsed());
    print!("\tmax: {:?}, min: {:?}, avg: {:?} nanoseconds\n\n", max.subsec_nanos(), min.subsec_nanos(), (average / iters).subsec_nanos());
}

fn benchmark(opers: u32) -> time::Duration {

    let start = std::time::Instant::now();
    //initialize psuedo random seed
    let mut rng = rand::thread_rng();

    // black box
    let accounts: Vec<Account> = hint::black_box(AccountSource::new(opers, rng.to_owned()).into_iter()
        .map(|mut account| {
            let payment = account.balance.min(account.current_bill);
            account.paid_amount += payment;
            account.paid_amount >>= 2;
            account.current_bill += account.current_bill - payment + rng.gen::<i32>() % 100;
            account.current_bill >>= 2;
            account.balance += rng.gen::<i32>() % 100;
            hint::black_box(account)
        }).collect());

    hint::black_box(accounts);

    //grab time
    start.elapsed()
}
