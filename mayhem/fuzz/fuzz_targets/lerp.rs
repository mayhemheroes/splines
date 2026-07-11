#![no_main]

use libfuzzer_sys::fuzz_target;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use splines::Interpolate;

// Additive Mayhem harness (libfuzzer-sys). Exercises the `Interpolate::lerp`
// implementation for f32 (the only type impl-ing Interpolate without extra
// features). Adapted to the upstream 0.2.1 API: lerp(a, b, t).
fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }

    // Create a seed from the input data.
    let mut seed = [0u8; 32];
    let seed_len = std::cmp::min(data.len(), 32);
    seed[..seed_len].copy_from_slice(&data[..seed_len]);

    // Instantiate a seeded random number generator.
    let mut rng = Pcg64::from_seed(seed);

    // Generate random values.
    let a = rng.gen::<f32>();
    let b = rng.gen::<f32>();
    let t: f32 = rng.gen_range(0.0..1.0);

    // Fuzz the lerp function.
    let _ = <f32 as Interpolate>::lerp(a, b, t);
});
