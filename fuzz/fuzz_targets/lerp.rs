use honggfuzz::fuzz;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use splines::Interpolate;

// Fuzzing

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 32 {
                return;
            }

            // Create a seed from the input data
            let mut seed = [0u8; 32];
            let seed_len = std::cmp::min(data.len(), 32);
            seed[..seed_len].copy_from_slice(&data[..seed_len]);

            // Instantiate a seeded random number generator
            let mut rng = Pcg64::from_seed(seed);

            // Generate random values
            let start = rng.gen::<f32>();
            let end = rng.gen::<f32>();
            let t: f32 = rng.gen_range(0.0..1.0);

            // Fuzz the lerp function
            let _ = Interpolate::lerp(t, start, end);
        });
    }
}