use num_bigint::{BigUint, RandBigInt};
use num_traits::{Zero, One};
use rand::{Rng, RngCore, CryptoRng};

fn MOD() -> BigUint {
    // TODO: Identify a suitable parameter
    BigUint::parse_bytes(b"1000000000100000000010000000001000000007", 10).unwrap()
}

#[derive(Debug, Clone)]
pub struct PublicKey(BigUint);

#[derive(Debug, Clone)]
pub struct SecretKey(BigUint);

#[derive(Debug, Clone)]
pub struct Key(pub PublicKey, pub SecretKey);

#[derive(Debug, Clone)]
pub struct Signature {
    pub rs: BigUint, // r + s
    pub fib_rm: BigUint,  // Fib(r + m)
    pub fib_r: BigUint // Fib(r)
}

fn multiply(a: &[[BigUint; 2]; 2], b: &[[BigUint; 2]; 2], q: &BigUint) -> [[BigUint; 2]; 2] {
    let a0 = a.get(0).unwrap();
    let a1 = a.get(1).unwrap();
    let b0 = b.get(0).unwrap();
    let b1 = b.get(1).unwrap();
    let a00 = a0.get(0).unwrap();
    let a01 = a0.get(1).unwrap();
    let a10 = a1.get(0).unwrap();
    let a11 = a1.get(1).unwrap();
    let b00 = b0.get(0).unwrap();
    let b01 = b0.get(1).unwrap();
    let b10 = b1.get(0).unwrap();
    let b11 = b1.get(1).unwrap();
    [
        [
            ((a00 * b00) % q + (a01 * b10) % q) % q, 
            ((a00 * b01) % q + (a01 * b11) % q) % q
        ],
        [
            ((a10 * b00) % q + (a11 * b10) % q) % q,
            ((a10 * b01) % q + (a11 * b11) % q) % q
        ]
    ]
}

pub fn fibonacci(n: BigUint, m: BigUint) -> BigUint {
    let mut _n = n.clone();
    let mut result_matrix: [[BigUint;2]; 2] = [[One::one(), Zero::zero()], [Zero::zero(), One::one()]];
    let mut fib_matrix: [[BigUint;2]; 2]= [[Zero::zero(), One::one()], [One::one(),  One::one()]];
    while (_n > Zero::zero()) {
        if (_n.bit(0)) {
            result_matrix = multiply(&result_matrix, &fib_matrix, &m);
        }
        _n = _n >> 1;
        fib_matrix = multiply(&fib_matrix.clone(), &fib_matrix, &m);
    }
    result_matrix[0][1].clone()
}

/**
 *  Fibonacci-Vajda Crypto System based on the following assumptions:
 * 
 *    1. Given `Fib(a) mod Q`, where `a` is any random scalar, and Fib is the standard fibonacci function, it's hard to infer a
 *    2. Given `Fib(a) mod Q`, `Fib(b) mod Q`, and `a + b`, it's hard to infer `a` and `b`
 *    3. Given `Fib(a) mod Q`, `Fib(b) mod Q`, and `Fib(a+b) mod Q`, it's hard to infer `a` and `b`
 * 
 *    PublicKey size: 32 bytes, Secret Key: 32 bytes
 *    Signature size: 96 bytes
 * 
 *    Algorithm:
 * 
 *    Parameters: Agree on the following parameters
 *        1. Q: A large number which results in a higher pisano period as fibonacci numbers exhibit cyclic period.
 * 
 *    Key Generation: 
 *        1. Generate a random scalar `s`, the secret value.
 *        2. The public key = Fib(s) mod Q
 * 
 *    Signing: Inputs: a message `m` and secret `s`
 *        1. Generate a random `r`, which is even
 *        2. Evaluate: sig=(r+s, Fib(r + m) mod Q, Fib(r) mod Q)
 * 
 *    Verification: Inputs: sig=(r+s, Fib(r+m) mod Q, Fib(r) mod Q), message `m`, public key: Fib(s) mod Q
 *        1. lhs = ( Fib(r + s) * Fib(r + m) )mod Q
 *        2. rhs = Fib(s)  * Fib(m) + Fib(r) * Fib(r + s + m), all operations mod Q, except scalar field addition
 *        3. if lhs == rhs, then verification is successful
 *        4. else: fail
 * 
 *    To prove:
 *         1. Vajda identity is a necessary and also sufficient condition.
 *         2. If needed, additional conditions like Honsberger's Identity can be verified to assert that data given by prover is valid, without much additional computation, but with extra 32 bytes of data in public key and signature.
 *              1. F(r+s) = F(r-1)F(s) + F(r)F(s+1), F(s+1)- needs to be obtained from additional 32 bytes in public key, and F(r-1) from signature.
 *              2. F(r+m) = F(r-1)F(m) + F(r)F(m+1), F(r-1) from signature and F(m+1) can be computed at the same time F(m) is being computed, and other data is already available.
 *
 */
#[derive(Clone)]
pub struct FibonacciVajdaCryptoSystem {
    pub q: BigUint, // MOD
    pub b: u64 // bit size
}

impl FibonacciVajdaCryptoSystem {
    pub fn new(q: BigUint, b: u64) -> Self {
        Self {
            q: q,
            b: b
        }
    }
    pub fn default() -> Self {
        Self {
            q: MOD(),
            b: 256
        }
    }

    /**
     * Generates a new key
     */
    pub fn generate_key<R: RngCore + CryptoRng>(self, rng: &mut R) -> Key {
        let random_num = rng.gen_biguint( self.b);
        let secret_key = SecretKey(random_num.clone());
        let public_key = PublicKey(fibonacci(random_num, self.q));
        Key (
            public_key,
            secret_key
        )
    }

    /**
     * Given a message (m), return the signature
     *     (r+s, Fib(r + m), Fib(r)), where
     *   r = random even number,
     *   s = secret key
     */
    pub fn sign<R: RngCore + CryptoRng>(self, secret_key: SecretKey, message: BigUint, rng: &mut R) -> Signature {
        let q = self.q;
        let b = self.b;
        let r: BigUint = ((rng.gen_biguint(b)) << 1); // has to be even
        let rs = r.clone() + secret_key.0;
        let rm = r.clone() + message;
        let fib_rm = fibonacci(rm, q.clone());
        let fib_r = fibonacci(r, q.clone());
        Signature { rs, fib_rm, fib_r}
    }


    /**
     * Given a signature (r + s, Fib(r + m), Fib(r)), verify the following Vajda's identity
     * 
     *     Fib(r + s) * Fib(r + m) == Fib(s) * Fib(m) + Fib(r) * Fib(r + s + m)
     * 
     * If the above equation is true, the signature is verified
     *   
     */
    pub fn verify(self, signature: Signature, public_key: PublicKey, message: BigUint) -> bool {
        let q = self.q;
        let rsm = signature.rs.clone() + message.clone();
        let fib_rs = fibonacci(signature.rs.clone(), q.clone());
        let lhs = (fib_rs * signature.fib_rm) % q.clone();
        let fib_rsm = fibonacci(rsm, q.clone());
        let fib_m = fibonacci(message.clone(), q.clone());
        let rhs = (((public_key.0 * fib_m) % q.clone()) + (signature.fib_r * fib_rsm) % q.clone()) % q.clone();
        lhs == rhs
    }   

}

