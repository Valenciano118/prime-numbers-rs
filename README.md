# prime-numbers-rs

Welcome to this project, I'm currently implementing multiple methods to calculate prime numbers. I'm mostly using it as a way of learning a bit more about the prime numbers while also helping a bit on my way on learning Rust and programming at a lower level.


I'll be updating this README explaining each one of the methods used and why one is faster than the other, while also providing some benchmarks to compare them.

I won't get too deep into it right now, but so far I'd say the biggest thing I've learnt from the project is `cache locality`. With the same implementation the segmented `Sieve of Eratosthenes` is about twice as fast as the regular `Sieve of Eratosthenes`, and you can even start to see the effect without getting to crazy big numbers. But I'll back that up with benchmarks in the future.

