use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fips202::{shake128, shake256};
use sha3;
use sha3::Digest;
use sha3::digest::{ExtendableOutput, XofReader, Update};
use sha3::digest as digest;

const STRING_1: &str = "";
const STRING_2: &str = "abc";
const STRING_3: &str = "in girum imus nocte et consumimur igni";
const STRING_4: &str = "4}2?-}5cu'L2`/2eq8tx'p6*j9/tLhL=3}DG7e?zQW};C_7J%P\";U<nkeeufs,+U";
const STRING_5: &str = "­¡¦§¨©«¬®¯°±´¶·¸»¿×÷ˆ˜–—‘’‚“”„†‡•…‰′″‹›‾⁄℘←↑→↓↔↵⇐⇑⇒⇓⇔∀∂∃∅∇∈∉∋∏∑−∗√∝∞∠∧∨∩∪∫∴\
                        ∼≅≈≠≡≤≥⊂⊃⊄⊆⊇⊕⊗⊥⋅⌈⌉⌊⌋〈〉◊♠♣♥♦⟨⟩¤¢£¥€¹½¼²³¾ªáÁàÀâÂåÅäÄãÃæÆçÇðÐéÉèÈêÊëËƒℑíÍìÌî\
                        ÎïÏñÑºóÓòÒôÔöÖõÕøØœŒℜšŠß™úÚùÙûÛüÜýÝÿŸþÞαΑβΒγΓδΔεΕζΖηΗθϑΘιΙκΚλΛμµΜνΝξΞοΟπϖ\
                        ΠρΡσΣςτΤυΥϒφΦχΧψΨωΩℵ";

const STRINGS: &[&str] = &[
    STRING_1,
    STRING_2,
    STRING_3,
    STRING_4,
    STRING_5,
];

const OUTLENS: &[usize] = &[
    0,
    1,
    5,
    12,
    256,
    512,
    666,
];


macro_rules! create_xof_benchmark {
    ($bench_name:ident, $group_name:expr, $xof:expr) => {
        pub fn $bench_name(c: &mut Criterion) {
            let mut group = c.benchmark_group($group_name);
            for string in STRINGS.iter() {
                for &outlen in OUTLENS.iter() {

                    // fips202 preparations
                    let mut output = vec![0u8; outlen];
                    let input = &mut string.as_bytes().to_owned();
                    let mut inlen = input.len();

                    group.bench_function(
                        format!("{}:{}", string, outlen),
                        |b| b.iter(|| {
                            $xof(&mut output, outlen, input, &mut inlen);
                        })
                    );
                }
            }
            group.finish();
        }
    }
}


macro_rules! create_comparison_benchmark {
    ($bench_name:ident, $group_name:expr, $hasher_cls:ty, $xof:expr) => {
        pub fn $bench_name(c: &mut Criterion) {
            let mut group = c.benchmark_group($group_name);
            for string in STRINGS.iter() {
                for &outlen in OUTLENS.iter() {

                    // fips202 preparations
                    let mut output = vec![0u8; outlen];
                    let input = &mut string.as_bytes().to_owned();
                    let mut inlen = input.len();

                    group.bench_function(
                        format!("fips202 {}:{}", string, outlen),
                        |b| b.iter(|| {
                            $xof(&mut output, outlen, input, &mut inlen);
                        })
                    );
                    group.bench_function(
                        format!("sha3 {}:{}", string, outlen),
                        |b| b.iter(|| {
                            let mut hasher = <$hasher_cls>::default();
                            hasher.update(string.as_bytes());
                            hasher.finalize_xof();
                        })
                    );
                }
            }
            group.finish();
        }
    }
}


create_xof_benchmark!(bench_shake128, "SHAKE128", fips202::shake128);
create_xof_benchmark!(bench_shake256, "SHAKE256", fips202::shake256);
create_comparison_benchmark!(compare_shake128, "SHAKE128 comparisons", sha3::Shake128, fips202::shake128);
create_comparison_benchmark!(compare_shake256, "SHAKE256 comparisons", sha3::Shake256, fips202::shake256);


criterion_group!(
    benches,
    bench_shake128,
    bench_shake256,
    compare_shake128,
    compare_shake256,
);

criterion_main!(benches);
