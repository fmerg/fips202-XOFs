# FIPS202/XOFs

Rust implementation of the SHAKE128 and SHAKE256 extendable-output functions
([XOFs](https://crypto.stackexchange.com/questions/30587/use-case-for-extendable-output-functions-xof-such-as-shake128-shake256))
from the [SHA3](https://en.wikipedia.org/wiki/SHA-3) standard as specified in
[FIPS202](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf).

*This is just an exercise focusing on low-level crypto. It is not intended to
be published as crate*.

The interface is crude. No "update/digest" semantics and message queue,
the whole bytestring must be already present in the buffer (see below).

### Aknowledgement

Certain ideas have been taken from
[this](https://github.com/XKCP/XKCP/blob/master/Standalone/CompactFIPS202/C/Keccak-readable-and-compact.c)
C implementation by the Keccak Team (see also
[here](https://github.com/XKCP/XKCP/blob/master/Standalone/CompactFIPS202/C/TweetableFIPS202.c)
for a more compact version), which readably reorganizes the tweetable
[TweetFIPS202](https://twitter.com/TweetFIPS202) implementation by
D. J. Bernstein, P. Schwabe and G. Van Assche. An optimized form of the
Keccak-F permutation function has been taken from the
[XMSS reference implementation](https://github.com/XMSS/xmss-reference/blob/master/keccak.c#L65).

### Application

```
$ cargo run --example [shake128|shake256]
```

### Usage

This is not available as a crate. Clone the project, include your program in `examples/`
and execute it with `cargo run`. For example, the following program computes the
SHAKE256-digest of length 12 of a UTF-8 encoded `"abc"`.

*examples/sample.rs*
```rust
use fips202::{shake128, shake256};

fn main() {

    // Preparations

    let input = &mut "abc".as_bytes().to_owned();   // the bytestring to be hashed
    let mut inlen = input.len();                    // length of original input
    let mut output = [0u8; 12];                     // will hold the final digest
    let outlen = output.len();                      // length of final digest

    // Hashing

    shake256(&mut output, outlen, input, &mut inlen);

    // Print result as hex string

    for byte in &output {
        print!("{:02x}", byte);
    }
    println!();
}
```

Running the program should produce the following result:

```bash
$ cargo run --example sample
483366601360a8771c686308
```

### Tests

```
$ cargo test
```

### Benchmarks

```
$ cargo bench
```
