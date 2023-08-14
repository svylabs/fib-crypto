I designed and implemented a simple crypto system for digital signatures based on Fibonacci numbers using the **Vadja's**[1]  identity. There system works based on some assumptions that are yet to be proven or disproven.

# Fibonacci-Vadja Crypto System

## Algorithm:
 
### Parameters: Agree on the following parameters

1. Q: A large number which results in a higher pisano period.
 
### Key Generation: 

1. Generate a random scalar `s`, the secret value.
2. The public key = Fib(s) mod Q
 
### Signing: Inputs: a message `m` and secret `s`

1. Generate a random `r`, which is even
2. Evaluate: sig=(r+s, Fib(r + m) mod Q, Fib(r) mod Q)
 
### Verification: Inputs: sig=(r+s, Fib(r+m) mod Q, Fib(r) mod Q), message `m`, public key: Fib(s) mod Q

1. lhs = ( Fib(r + s) * Fib(r + m) )mod Q
2. rhs = Fib(s)  * Fib(m) + Fib(r) * Fib(r + s + m), all operations mod Q, except scalar field addition
3. if lhs == rhs, then verification is successful
4. else: fail

## Assumptions (To be proven)
 
1. Given `Fib(a) mod Q`, where `a` is any random scalar, and Fib is the standard fibonacci function, it's hard to infer a
2. Given `Fib(a) mod Q`, `Fib(b) mod Q`, and `a + b`, it's hard to infer `a` and `b`
3. Given `Fib(a) mod Q`, `Fib(b) mod Q`, and `Fib(a+b) mod Q`, it's hard to infer `a` and `b`
4. Vadja identity is a necessary and also sufficient condition.
5. If needed, additional conditions like Honsberger's Identity can be verified to assert that data given by prover is valid, without much additional computation, but with extra 32 bytes of data in public key and signature.
    1. F(r+s) = F(r-1)F(s) + F(r)F(s+1), F(s+1)- needs to be obtained from additional 32 bytes in public key, and F(r-1) from signature.
    2. F(r+m) = F(r-1)F(m) + F(r)F(m+1), F(r-1) from signature and F(m+1) can be computed at the same time F(m) is being computed, and other data is already available.

## Key sizes

PublicKey size: 32 bytes, Secret Key: 32 bytes
Signature size: 96 bytes

In the stronger case, where we verify Honsberger's identity during signature verification, the key sizes are

Public Key: 64 bytes
Signature size: 128 bytes

## References

1. S. Vajda, Fibonacci and Lucas numbers, and the golden section: theory and applications, Dover Press, (2008).

