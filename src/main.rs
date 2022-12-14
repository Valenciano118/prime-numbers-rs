#![feature(test, int_roundings)]
use std::{
    cmp::min,
    collections::HashSet,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

use parking_lot::RwLock;

const NUM_OF_PRIMES: u64 = 1_000_000_000;

//Resource for optimizing Sieve of Eratosthenes
//http://warp.povusers.org/programming/sieve_of_eratosthenes.html
fn main() {
    // let instant = Instant::now();
    // let res = single_threaded_prime_naive(NUM_OF_PRIMES);
    // let elapsed_single = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    // println!("It took {elapsed_single} seconds to calculate {res} primes (single) on the first {NUM_OF_PRIMES}  ");

    let instant = Instant::now();
    let res = sieve_of_eratosthenes(NUM_OF_PRIMES).len();
    let elapsed_sieve = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_sieve} seconds to calculate {res} primes (sieve_eratosthenes) on the first {NUM_OF_PRIMES}  ");

    let instant = Instant::now();
    let res = segmented_sieve_of_eratosthenes(NUM_OF_PRIMES);
    let elapsed_segmented = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_segmented} seconds to calculate {res} primes (segmented_sieve_eratosthenes) on the first {NUM_OF_PRIMES} ");

    let instant = Instant::now();
    let res = multi_segmented_sieve_of_eratosthenes(NUM_OF_PRIMES);
    let elapsed_multi = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_multi} seconds to calculate {res} primes (multi_segmented_sieve_eratosthenes) on the first {NUM_OF_PRIMES} ");

    // let instant = Instant::now();
    // let res = multi_sieve_of_eratostehenes(NUM_OF_PRIMES as usize);
    // let elapsed_multi = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    // println!("It took {elapsed_multi} seconds to calculate {res} primes (multi_sieve_eratosthenes) on the first {NUM_OF_PRIMES} ");

    let instant = Instant::now();
    let res = sieve_of_atkin(NUM_OF_PRIMES);
    let elapsed_atkin = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_atkin} seconds to calculate {res} primes (sieve_atkin) on the first {NUM_OF_PRIMES} ");

    let instant = Instant::now();
    let res = single_wheel_mod30(NUM_OF_PRIMES);
    let elapsed_single_wheel = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed_single_wheel} seconds to calculate {res} primes (single_wheel_mod30) on the first {NUM_OF_PRIMES} ");
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

fn sieve_of_eratosthenes(n: u64) -> Vec<u64> {
    let mut primes_list: Vec<bool> = vec![true; n as usize + 1];

    let mut p = 2u64;
    primes_list[0] = false;
    primes_list[1] = false;

    let n_sqrt: u64 = (n as f64).sqrt().ceil() as u64;

    while p <= n_sqrt {
        if primes_list[p as usize] {
            let mut i = p.pow(2);
            while i <= n {
                primes_list[i as usize] = false;
                i += p
            }
        }
        p += 1;
    }
    let res: Vec<u64> = primes_list
        .iter()
        .enumerate()
        .filter(|(_, &x)| x)
        .map(|(index, _)| index as u64)
        .collect();
    res
}

fn segmented_sieve_of_eratosthenes(n: u64) -> u64 {
    let limit = (n as f64).sqrt() as u64;

    let primes_smaller_than_sqrt: Vec<u64> = sieve_of_eratosthenes(limit);
    let primes_length = primes_smaller_than_sqrt.len();
    let mut prime_count = primes_length;

    let mut lower_limit = limit;
    let mut upper_limit = limit * 2;

    while lower_limit < n {
        if upper_limit >= n {
            upper_limit = n;
        }

        let mut current_segment_primes: Vec<bool> = vec![true; limit as usize];

        for i in 0..primes_length {
            let mut low_lim = u64::div_floor(lower_limit, primes_smaller_than_sqrt[i])
                * primes_smaller_than_sqrt[i];
            if low_lim < lower_limit {
                low_lim += primes_smaller_than_sqrt[i];
            }

            for j in (low_lim..upper_limit).step_by(primes_smaller_than_sqrt[i] as usize) {
                current_segment_primes[(j - lower_limit) as usize] = false;
            }
        }

        for i in lower_limit..upper_limit {
            if current_segment_primes[(i - lower_limit) as usize] {
                prime_count += 1;
            }
        }
        lower_limit += limit;
        upper_limit += limit;
    }
    prime_count as u64
}

fn multi_segmented_sieve_of_eratosthenes(n: u64) -> u64 {
    let limit = (n as f64).sqrt() as u64;

    let shared_primes_smaller_than_sqrt: Arc<Vec<u64>> = Arc::new(sieve_of_eratosthenes(limit));
    let primes_length = shared_primes_smaller_than_sqrt.len();
    let shared_prime_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(primes_length));

    let num_threads = std::thread::available_parallelism().unwrap().get();
    let mut join_handler: Vec<JoinHandle<()>> = vec![];

    for id in 1..=num_threads {
        let primes_smaller_than_sqrt = Arc::clone(&shared_primes_smaller_than_sqrt);
        let shared_prime_count = Arc::clone(&shared_prime_count);
        let handle = thread::spawn(move || {
            let mut lower_limit = limit * id as u64;
            let mut upper_limit = lower_limit + limit;
            let mut local_prime_count = 0;

            while lower_limit < n {
                if upper_limit > n {
                    upper_limit = n;
                }

                let mut current_segment_primes: Vec<bool> = vec![true; limit as usize];

                for i in 0..primes_length {
                    let mut low_lim = u64::div_floor(lower_limit, primes_smaller_than_sqrt[i])
                        * primes_smaller_than_sqrt[i];
                    if low_lim < lower_limit {
                        low_lim += primes_smaller_than_sqrt[i];
                    }

                    for j in (low_lim..upper_limit).step_by(primes_smaller_than_sqrt[i] as usize) {
                        current_segment_primes[(j - lower_limit) as usize] = false;
                    }
                }

                for i in lower_limit..upper_limit {
                    if current_segment_primes[(i - lower_limit) as usize] {
                        local_prime_count += 1;
                    }
                }
                lower_limit += limit * num_threads as u64;
                upper_limit = lower_limit + limit;
            }
            let mut prime_count = shared_prime_count.lock().unwrap();
            *prime_count += local_prime_count;
        });
        join_handler.push(handle);
    }

    for handle in join_handler {
        handle.join().unwrap();
    }

    let res = *shared_prime_count.lock().unwrap() as u64;
    res
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

fn single_wheel_mod30(num: u64) -> u64 {
    let mut primes_list: Vec<bool> = vec![false; num as usize + 1];

    let wheel: Vec<usize> = vec![7, 11, 13, 17, 19, 23, 29, 31];

    for w in wheel {
        for i in (w..=num as usize).step_by(30) {
            primes_list[i] = true;
        }
    }

    for index in 0..primes_list.len() {
        if primes_list[index] {
            let mut i = 2 * index;
            while i < num as usize {
                primes_list[i] = false;
                i += index;
            }
        }
    }

    primes_list[2] = true;
    primes_list[3] = true;
    primes_list[5] = true;
    let res: u64 = primes_list.iter().filter(|&&x| x).count() as u64;
    res
}


#[cfg(test)]
mod bench {
    use super::*;
    extern crate test;
    use test::Bencher;

    const NUM_OF_PRIMES: u64 = 1_000_000;

    #[bench]
    fn bench_eratosthenes_1_000_000(b: &mut Bencher) {
        b.iter(|| sieve_of_eratosthenes(NUM_OF_PRIMES));
    }
    fn bench_eratosthenes_multi_1_000_000(b: &mut Bencher) {
        b.iter(|| multi_sieve_of_eratostehenes(NUM_OF_PRIMES as usize));
    }
    #[bench]
    fn bench_atking_1_000_000(b: &mut Bencher) {
        b.iter(|| sieve_of_atkin(NUM_OF_PRIMES));
    }
    #[bench]
    fn bench_naive_single_1_000_000(b: &mut Bencher) {
        b.iter(|| single_threaded_prime_naive(NUM_OF_PRIMES));
    }
    #[bench]
    fn bench_naive_multi_std_1_000_000(b: &mut Bencher) {
        b.iter(|| multi_threaded_prime_naive_std_mutex(NUM_OF_PRIMES));
    }
    #[bench]
    fn bench_naive_multi_parking_1_000_000(b: &mut Bencher) {
        b.iter(|| multi_threaded_prime_naive_parking_lot_mutex(NUM_OF_PRIMES));
    }
}
