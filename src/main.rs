use fib_crypto::{FibonacciVajdaCryptoSystem, fibonacci};
use num_bigint::BigUint;

const MOD: u64 = 1000000007;

fn fib(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    if (n == 0) {
        return 0
    }
    for i in 1..n {
        c = (a + b) % MOD;
        a = b;
        b = c
    }
    b
}

fn main() {
    use std::time::Instant;
    let crypto = FibonacciVajdaCryptoSystem::default();
    let mut rng = rand::thread_rng();
    let key = crypto.clone().generate_key(&mut rng);
    println!("key={:?}", key);
    let q=  crypto.clone().q;
    let message = BigUint::parse_bytes(b"100000000", 10).unwrap();
    let fib_10000 = fibonacci(message.clone(), q.clone());
    //println!("{} {}  {} {}", secret, public, message.clone(), fib_10000);
    for i in 1..500 {
        let now = Instant::now();
        let crypto = FibonacciVajdaCryptoSystem::default();
        let message = BigUint::parse_bytes(b"100000000", 10).unwrap();
        let signature = crypto.clone().sign(key.clone().1, message.clone(), &mut rng);
        let sign = now.elapsed();
        let verification = crypto.verify(signature.clone(), key.clone().0, message);
        println!("{:?} {:?} {}", signature, key.clone().0, verification);
        let verify = now.elapsed();
        println!("Sign: {:.2?}, Verify: {:.2?}", sign.as_millis(), verify.as_millis() - sign.as_millis());
    }
}
