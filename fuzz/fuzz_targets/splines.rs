use honggfuzz::fuzz;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use splines::{Interpolation, Key, Spline};

// Fuzzing

fn main() {

    loop {
        fuzz!(|data: &[u8]| {
            // Create a seed from the input data
            let mut seed = [0u8; 32];
            for (dst, src) in seed.iter_mut().zip(data.iter()) {
                *dst = *src;
            }

            // Instantiate a seeded random number generator
            let mut rng = StdRng::from_seed(seed);

            // Fuzz f64
            let start_step_f64 = Key::new(
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                Interpolation::Step(rng.gen::<f64>()),
            );

            let start_linear_f64 = Key::new(
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                Interpolation::Linear,
            );

            let start_cosine_f64 = Key::new(
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                Interpolation::Cosine,
            );

            let start_catmullrom_f64 = Key::new(
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                Interpolation::CatmullRom,
            );

            let end_key_f64 = Key::new(
                rng.gen::<f64>(),
                rng.gen::<f64>(),
                Interpolation::default()
            );

            let spline_step_f64 = Spline::<f64, _>::from_vec(vec![start_step_f64, end_key_f64.clone()]);
            let spline_linear_f64 = Spline::<f64, _>::from_vec(vec![start_linear_f64, end_key_f64.clone()]);
            let spline_cosine_f64 = Spline::<f64, _>::from_vec(vec![start_cosine_f64, end_key_f64.clone()]);
            let spline_catmullrom_f64 = Spline::<f64, _>::from_vec(vec![start_catmullrom_f64, end_key_f64]);

            let tf64 = rng.gen::<f64>();
            let _ = spline_step_f64.sample(tf64);
            let _ = spline_step_f64.clamped_sample(tf64);
            let _ = spline_step_f64.sample_with_key(tf64);
            let _ = spline_step_f64.clamped_sample_with_key(tf64);
            let _ = spline_linear_f64.sample(tf64);
            let _ = spline_linear_f64.clamped_sample(tf64);
            let _ = spline_linear_f64.sample_with_key(tf64);
            let _ = spline_linear_f64.clamped_sample_with_key(tf64);
            let _ = spline_cosine_f64.sample(tf64);
            let _ = spline_cosine_f64.clamped_sample(tf64);
            let _ = spline_cosine_f64.sample_with_key(tf64);
            let _ = spline_cosine_f64.clamped_sample_with_key(tf64);
            let _ = spline_catmullrom_f64.sample(tf64);
            let _ = spline_catmullrom_f64.clamped_sample(tf64);
            let _ = spline_catmullrom_f64.sample_with_key(tf64);
            let _ = spline_catmullrom_f64.clamped_sample_with_key(tf64);

            // Fuzz f32
            let start_step_f32 = Key::new(
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                Interpolation::Step(rng.gen::<f32>()),
            );
            
            let start_linear_f32 = Key::new(
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                Interpolation::Linear,
            );
            
            let start_cosine_f32 = Key::new(
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                Interpolation::Cosine,
            );
            
            let start_catmullrom_f32 = Key::new(
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                Interpolation::CatmullRom,
            );
            
            let end_key_f32 = Key::new(
                rng.gen::<f32>(),
                rng.gen::<f32>(),
                Interpolation::default(),
            );
            
            let spline_step_f32 = Spline::<f32, _>::from_vec(vec![start_step_f32, end_key_f32.clone()]);
            let spline_linear_f32 = Spline::<f32, _>::from_vec(vec![start_linear_f32, end_key_f32.clone()]);
            let spline_cosine_f32 = Spline::<f32, _>::from_vec(vec![start_cosine_f32, end_key_f32.clone()]);
            let spline_catmullrom_f32 = Spline::<f32, _>::from_vec(vec![start_catmullrom_f32, end_key_f32]);
            
            let tf32 = rng.gen::<f32>();
            let _ = spline_step_f32.sample(tf32);
            let _ = spline_step_f32.clamped_sample(tf32);
            let _ = spline_step_f32.sample_with_key(tf32);
            let _ = spline_step_f32.clamped_sample_with_key(tf32);
            let _ = spline_linear_f32.sample(tf32);
            let _ = spline_linear_f32.clamped_sample(tf32);
            let _ = spline_linear_f32.sample_with_key(tf32);
            let _ = spline_linear_f32.clamped_sample_with_key(tf32);
            let _ = spline_cosine_f32.sample(tf32);
            let _ = spline_cosine_f32.clamped_sample(tf32);
            let _ = spline_cosine_f32.sample_with_key(tf32);
            let _ = spline_cosine_f32.clamped_sample_with_key(tf32);
            let _ = spline_catmullrom_f32.sample(tf32);
            let _ = spline_catmullrom_f32.clamped_sample(tf32);
            let _ = spline_catmullrom_f32.sample_with_key(tf32);
            let _ = spline_catmullrom_f32.clamped_sample_with_key(tf32);
        });
    }
}