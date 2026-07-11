#![no_main]

use libfuzzer_sys::fuzz_target;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use splines::{Interpolation, Key, Spline};

// Additive Mayhem harness (libfuzzer-sys). Builds splines over f32 (the only
// type impl-ing Interpolate without extra features) across every interpolation
// mode and samples them at fuzzed times. Adapted to the upstream 0.2.1 API:
// Key::new(t: f32, value: T, Interpolation), Interpolation::Step(f32),
// Spline<T> (single type param), Spline::sample/clamped_sample.
fuzz_target!(|data: &[u8]| {
    if data.len() < 32 {
        return;
    }

    // Create a seed from the input data.
    let mut seed = [0u8; 32];
    for (dst, src) in seed.iter_mut().zip(data.iter()) {
        *dst = *src;
    }

    // Instantiate a seeded random number generator.
    let mut rng = Pcg64::from_seed(seed);

    // Random control-point times/values.
    let t0 = rng.gen::<f32>();
    let t1 = rng.gen::<f32>();
    let t2 = rng.gen::<f32>();
    let t3 = rng.gen::<f32>();
    let v0 = rng.gen::<f32>();
    let v1 = rng.gen::<f32>();
    let v2 = rng.gen::<f32>();
    let v3 = rng.gen::<f32>();
    let step_threshold = rng.gen::<f32>();

    let sample_t = rng.gen::<f32>();

    // One spline per interpolation mode. CatmullRom needs >= 4 keys, so give
    // every spline four keys to keep the exercised paths broad.
    let modes = [
        Interpolation::Step(step_threshold),
        Interpolation::Linear,
        Interpolation::Cosine,
        Interpolation::CatmullRom,
    ];

    for mode in modes.iter().copied() {
        let keys = vec![
            Key::new(t0, v0, mode),
            Key::new(t1, v1, mode),
            Key::new(t2, v2, mode),
            Key::new(t3, v3, Interpolation::default()),
        ];
        let spline = Spline::from_vec(keys);

        let _ = spline.sample(sample_t);
        // clamped_sample panics with no keys; we always have keys here.
        let _ = spline.clamped_sample(sample_t);
        let _ = spline.keys();
        for _k in &spline {
            // exercise the Iter impl
        }
    }
});
