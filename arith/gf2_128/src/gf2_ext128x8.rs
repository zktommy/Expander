#[cfg(target_arch = "aarch64")]
pub(crate) mod neon;
#[cfg(target_arch = "aarch64")]
pub type GF2_128x8 = neon::NeonGF2_128x8;

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
mod avx512;
#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
pub type GF2_128x8 = avx512::AVX512GF2_128x8;

// Fallback, use avx2
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx512f")))]
mod avx256;
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx512f")))]
pub type GF2_128x8 = avx256::AVX256GF2_128x8;

impl Ord for GF2_128x8 {
    #[inline(always)]
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        unimplemented!("Ord for GF2_128x8 is not supported")
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for GF2_128x8 {
    #[inline(always)]
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        unimplemented!("PartialOrd for GF2_128x8 is not supported")
    }
}
