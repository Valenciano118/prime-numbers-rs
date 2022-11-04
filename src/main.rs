use std::time::Instant;

const NUM_OF_PRIMES: u32 = 1_000_000;
fn main() {
    let instant = Instant::now();
    single_threaded_prime(NUM_OF_PRIMES);
    let elapsed = instant.elapsed().as_nanos() as f64 / 1_000_000_000.0;

    println!("It took {elapsed} seconds to calculate {NUM_OF_PRIMES} primes ");
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

fn is_prime(num: u32) -> bool {
    let sqrt = (num as f64).sqrt() as u32;
    if num > 1 {
        for i in 2..sqrt {
            if (num % i) == 0 {
                return false;
            }
        }
    }
    true
}
