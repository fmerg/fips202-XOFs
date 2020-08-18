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
