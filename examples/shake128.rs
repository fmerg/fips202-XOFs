/// Prints as hexstring the SHAKE128-digest
/// of length 12 of the user provided input

use std::io::{self, Read};
use fips202;

const outlen: usize = 12;

fn hexprint(sum: &[u8], name: &str) {
    for byte in sum {
        print!("{:02x}", byte);
    }
    println!("\t{}", name);
}

fn collect<R: Read>(reader: &mut R) -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    buffer.trim_end().to_string()
}

fn main() {
    println!("\nProvide input:");
    let string = collect::<_>(&mut io::stdin());
    let mut input = string.as_bytes().to_owned();
    let mut inlen = input.len();
    let mut output = [0u8; outlen];

    fips202::shake128(&mut output, outlen, &mut input, &mut inlen);
    hexprint(&output, "SHAKE128");
}
