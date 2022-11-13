use std::{
    cmp::min,
    collections::HashSet,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

use parking_lot::RwLock;

const NUM_OF_PRIMES: u64 = 1_000_000_000;
fn main() {
    // let instant = Instant::now();
    // let res = single_threaded_prime_naive(NUM_OF_PRIMES);
    // let elapsed_single = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    // println!("It took {elapsed_single} seconds to calculate {res} primes (single) on the first {NUM_OF_PRIMES}  ");

    let instant = Instant::now();
    let res = sieve_of_eratosthenes(NUM_OF_PRIMES);
    let elapsed_sieve = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_sieve} seconds to calculate {res} primes (sieve_eratosthenes) on the first {NUM_OF_PRIMES}  ");

    // let instant = Instant::now();
    // let res = multi_threaded_prime_naive_std_mutex(NUM_OF_PRIMES);
    // let elapsed_multi = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    // println!("It took {elapsed_multi} seconds to calculate {res} primes (multi_std_mutex) on the first {NUM_OF_PRIMES} ");

    // let instant = Instant::now();
    // let res = multi_threaded_prime_naive_parking_lot_mutex(NUM_OF_PRIMES);
    // let elapsed_multi = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    // println!("It took {elapsed_multi} seconds to calculate {res} primes (multi_parking_lot_mutex) on the first {NUM_OF_PRIMES} ");

    // let instant = Instant::now();
    // let res = multi_sieve_of_eratostehenes(NUM_OF_PRIMES as usize);
    // let elapsed_multi = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    // println!("It took {elapsed_multi} seconds to calculate {res} primes (multi_sieve_eratosthenes) on the first {NUM_OF_PRIMES} ");

    let instant = Instant::now();
    let res = sieve_of_atkin(NUM_OF_PRIMES);
    let elapsed_atkin = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_atkin} seconds to calculate {res} primes (sieve_atkin) on the first {NUM_OF_PRIMES} ");
}

fn single_threaded_prime_naive(num_of_primes: u64) -> u64 {
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

fn multi_threaded_prime_naive_std_mutex(num_of_primes: u64) -> u64 {
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

    let x = *counter.lock().unwrap();
    x
}

fn multi_threaded_prime_naive_parking_lot_mutex(num_of_primes: u64) -> u64 {
    let mut join_handler: Vec<JoinHandle<()>> = vec![];

    let available_cores = std::thread::available_parallelism().unwrap().get() as u64;
    let counter = Arc::new(parking_lot::Mutex::new(0));

    let tam_bloq = (num_of_primes + available_cores - 1) / available_cores;
    for id in 0..available_cores {
        let total_primes = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let start = tam_bloq * id;
            let fin = min(start + tam_bloq, num_of_primes);
            for num in start..fin {
                if is_prime(num) {
                    let mut temp_counter = total_primes.lock();

                    *temp_counter += 1;
                }
            }
        });
        join_handler.push(handle);
    }

    for handle in join_handler {
        handle.join().unwrap();
    }

    let x = *counter.lock();
    x
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
    primes_list.iter().skip(2).filter(|&&x| x).count() as u64 //It starts calculating from 2, so we remove the first 2 elements
}

fn multi_sieve_of_eratostehenes(n: usize) -> u64 {
    let primes: Arc<RwLock<Vec<bool>>> = Arc::new(RwLock::new(vec![true; n + 1]));

    let m = std::thread::available_parallelism().unwrap().get();

    let seen_primes_shared: Arc<RwLock<HashSet<usize>>> = Arc::new(RwLock::new(HashSet::new()));

    let mut first_m_primes: Vec<usize> = vec![];

    let mut count = 2;
    while first_m_primes.len() < m {
        if is_prime(count) {
            first_m_primes.push(count as usize)
        }
        count += 1;
    }

    let mut join_handler: Vec<JoinHandle<()>> = vec![];
    for initial_value in first_m_primes {
        let primes_map = Arc::clone(&primes);

        let seen_primes = Arc::clone(&seen_primes_shared);
        let mut p = initial_value;
        let handle = thread::spawn(move || {
            while p.pow(2) <= n {
                if !seen_primes.read().contains(&p) && primes_map.read()[p] {
                    seen_primes.write().insert(p);
                    let mut i = p.pow(2);
                    while i <= n {
                        primes_map.write()[i] = false;
                        i += p
                    }
                }
                p += 1;
            }
        });
        join_handler.push(handle)
    }
    for handle in join_handler {
        handle.join().unwrap();
    }

    let x = primes.read().iter().filter(|&v| *v).count() as u64 - 2;
    x
}

fn sieve_of_atkin(n: u64) -> u64 {
    let mut sieve: Vec<bool> = vec![false; n as usize + 1];

    if n > 2 {
        sieve[2] = true;
    }
    if n > 3 {
        sieve[3] = true;
    }
    let mut x = 1u64;
    let mut y: u64;
    let mut x_2 = 1u64;
    let mut y_2: u64;

    while x_2 <= n {
        y = 1;
        y_2 = 1;
        while y_2 <= n {
            let mut v: u64 = (4 * x_2) + (y_2);

            if v <= n && (v % 12 == 1 || v % 12 == 5) {
                sieve[v as usize] ^= true;
            }

            v -= x_2;
            if v <= n && v % 12 == 7 {
                sieve[v as usize] ^= true;
            }

            v -= 2 * y_2;

            if x > y && v <= n && v % 12 == 11 {
                sieve[v as usize] ^= true;
            }
            y += 1;
            y_2 = y * y;
        }
        x += 1;
        x_2 = x * x
    }

    let mut r = 5u64;
    while r * r <= n {
        if sieve[r as usize] {
            let mut i = r * r;
            while i <= n {
                sieve[i as usize] = false;
                i += r * r;
            }
        }
        r += 1;
    }

    let x = sieve.iter().filter(|&v| *v).count() as u64;
    x
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
