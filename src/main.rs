use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Instant,
};

const NUM_OF_PRIMES: u32 = 1_000_000;
fn main() {
    let instant = Instant::now();
    single_threaded_prime(NUM_OF_PRIMES);
    let elapsed_single = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_single} seconds to calculate {NUM_OF_PRIMES} primes (single) ");

    let instant = Instant::now();
    multi_threaded_prime(NUM_OF_PRIMES);
    let elapsed_multi = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_multi} seconds to calculate {NUM_OF_PRIMES} primes (multi) ");

    println!("The speedup was of {}", elapsed_single / elapsed_multi);
}

fn single_threaded_prime(num_of_primes: u32) {
    let mut curr_num_of_primes = 0u32;
    let mut curr_num = 2u32;

    while curr_num_of_primes < num_of_primes {
        if is_prime(curr_num) {
            curr_num_of_primes += 1;
        }
        curr_num += 1;
    }
}

fn multi_threaded_prime(num_of_primes: u32) {
    let mut join_handler: Vec<JoinHandle<()>> = vec![];

    let available_cores = std::thread::available_parallelism().unwrap().get() as u32;

    let calculated_primes = Arc::new(Mutex::new(0u32));
    let finished = Arc::new(AtomicBool::new(false));

    for id in 0..available_cores {
        let calculated_primes = Arc::clone(&calculated_primes);
        let finished = Arc::clone(&finished);
        let handle = thread::spawn(move || {
            let mut num = id * available_cores;
            while !finished.load(Ordering::Relaxed) {
                if is_prime(num) {
                    let mut count = calculated_primes.lock().unwrap();
                    //println!("{num}");
                    *count += 1;
                    if *count == num_of_primes {
                        finished.swap(true, Ordering::Relaxed);
                    }
                }
                num += 1;
            }
        });

        join_handler.push(handle);
    }

    for handle in join_handler {
        handle.join().unwrap();
    }
}

fn is_prime(num: u32) -> bool {
    let sqrt = (num as f64).sqrt() as u32;
    if num > 1 {
        for i in 2..sqrt {
            if (num % i) == 0 {
                return false;
            }
        }
        return true;
    }
    false
}
