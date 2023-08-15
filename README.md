I designed and implemented a toy crypto system for digital signatures based on Fibonacci numbers using the **Vajda's**[1]  identity. The system works based on some assumptions that are yet to be proven or disproven.

# Fibonacci-Vajda Crypto System

## Fibonacci Commitment Algorithm

Let `m` be a message, then fibonacci commitment `c` is defined as

```
   c = Fib(m) mod Q
```

where c is the fibonacci number mod Q at index m.

## Digital Signature Algorithm:
 
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
      - The problem reduces to finding a = log(F) to the base 'phi', where phi = (1 + sqrt(5)) / 2
2. Given `Fib(a) mod Q`, `Fib(b) mod Q`, and `a + b`, it's hard to infer `a` and `b`
3. Given `Fib(a) mod Q`, `Fib(b) mod Q`, and `Fib(a+b) mod Q`, it's hard to infer `a` and `b`
4. Vajda's identity is a necessary and also sufficient condition.
```
    Vajda's identity states that

    Fib(r+s) * Fib(r + m) - Fib(r) * Fib(r + s + m) = (-1)^r * Fib(s) * Fib(m)
```

The above identity is modifed for this algorithm as below:

As r is chosen as an even number, -1^r is always 1, and instead of doing a subtraction, we verify

```
  Fib(r + s) * Fib(r + m) = Fib(s) * Fib(m) + Fib(r) * Fib(r + s + m)
```

    The variables are computed as below: 
        1. Fib(r + s) - computed from `r + s`, supplied by the prover as part of signature
        2. Fib(r + m) is supplied by the prover as part of signature
        3. Fib(s) - the public key
        4. Fib(m) - Can be computed from message `m`
        5. Fib(r) is supplied by the user as a part of signature.
        6. Fib(r + s + m) - computed by adding `m` to `r + s` (from signature) and taking Fib mod Q

5. If needed, additional conditions like Honsberger's Identity can be verified to assert that data - `r + s`, `F(r + m)` and `F(r)` given by prover is valid, adding only slight computation overhead, but with extra 32 bytes of data in public key and signature.
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

