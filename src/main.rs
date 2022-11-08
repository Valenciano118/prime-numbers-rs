use std::{
    cmp::min,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

const NUM_OF_PRIMES: u64 = 1_000_000;
fn main() {
    let instant = Instant::now();
    let res = single_threaded_prime(NUM_OF_PRIMES);
    let elapsed_single = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_single} seconds to calculate {res} primes (single) on the first {NUM_OF_PRIMES}  ");

    let instant = Instant::now();
    let res = sieve_of_eratosthenes(NUM_OF_PRIMES);
    let elapsed_sieve = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_sieve} seconds to calculate {res} primes (sieve) on the first {NUM_OF_PRIMES}  ");

    let instant = Instant::now();
    let res = multi_threaded_prime(NUM_OF_PRIMES);
    let elapsed_multi = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_multi} seconds to calculate {res} primes (multi) on the first {NUM_OF_PRIMES} ");

    println!("The speedup was of {}", elapsed_sieve / elapsed_multi);
}

fn single_threaded_prime(num_of_primes: u64) -> u64 {
    let mut curr_num_of_primes = 0u64;
    let mut curr_num = 0u64;

    while curr_num < num_of_primes {
        if is_prime(curr_num) {
            curr_num_of_primes += 1;
        }
        curr_num += 1;
    }
    curr_num_of_primes
}

fn multi_threaded_prime(num_of_primes: u64) -> u64 {
    let mut join_handler: Vec<JoinHandle<()>> = vec![];

    let available_cores = std::thread::available_parallelism().unwrap().get() as u64;
    let counter = Arc::new(Mutex::new(0));

    let tam_bloq = (num_of_primes + available_cores - 1) / available_cores;
    for id in 0..available_cores {
        let total_primes = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let start = tam_bloq * id;
            let fin = min(start + tam_bloq, num_of_primes);
            for num in start..fin {
                if is_prime(num) {
                    let mut temp_counter = total_primes.lock().unwrap();

                    *temp_counter += 1;
                }
            }
        });
        join_handler.push(handle);
    }

    for handle in join_handler {
        handle.join().unwrap();
    }

    *counter.clone().lock().unwrap()
}

fn is_prime(num: u64) -> bool {
    let sqrt = (num as f64).sqrt() as u64;
    if num < 2 {
        return false;
    }
    for i in 2..=sqrt {
        if (num % i) == 0 {
            return false;
        }
    }
    true
}

fn sieve_of_eratosthenes(n: u64) -> u64 {
    let mut primes_list: Vec<bool> = vec![true; n as usize + 1];

    let mut p = 2u64;

    while p.pow(2) <= n {
        if primes_list[p as usize] {
            let mut i = p.pow(2);
            while i <= n {
                primes_list[i as usize] = false;
                i += p
            }
        }
        p += 1;
    }
    // for (i, elem) in primes_list.clone().iter().enumerate() {
    //     if *elem {
    //         println!("{i} is prime");
    //     }
    // }
    primes_list.iter().skip(2).filter(|&&x| x).count() as u64 //It starts calculating from 2, so we remove the first 2 elements
}
