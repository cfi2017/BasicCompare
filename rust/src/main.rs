use std::{hint};


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
        let diff = hint::black_box(programrust::benchmark(opers));
        if diff < min { min = diff };
        if diff > max { max = diff };
        average += diff;
    });

    //print result
    println!("Done! after {:?}", total.elapsed());
    print!("\tmax: {:?}, min: {:?}, avg: {:?} nanoseconds\n\n", max.subsec_nanos(), min.subsec_nanos(), (average / iters).subsec_nanos());
}
