use std::collections::HashMap;
use std::io::{self, Read};
use fips202;
use sha3;
use sha3::Digest;
use sha3::digest::{ExtendableOutput, XofReader, Update};
use sha3::digest as digest;


macro_rules! create_comparison_test {
    ($test_name:ident, $hasher_cls:ty, $xof:expr, $string:expr) => {
        #[test]
        fn $test_name() {
            let mut hasher = <$hasher_cls>::default();
            hasher.update($string.as_bytes());
            let mut digest = hasher.finalize_xof();
            let mut output_1 = [0u8; 12];
            Read::read(&mut digest, &mut output_1);

            let input = &mut $string.as_bytes().to_owned();
            let mut output_2 = [0u8; 12];
            let mut inlen = input.len();
            $xof(&mut output_2, 12, input, &mut inlen);

            assert_eq!(output_1, output_2);
        }
    };
 }


create_comparison_test!(compare_shake128, sha3::Shake128, fips202::shake128, "abc");
create_comparison_test!(compare_shake256, sha3::Shake256, fips202::shake256, "abc");
