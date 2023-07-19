use rand::Rng;
use stopwatch::Stopwatch;
use num_bigint::BigUint;
use crate::bn254::get_fr;
use crate::gpu::single_buffer_compute;
use crate::wgsl::concat_files;
use crate::utils::{ bigints_to_bytes, u32s_to_bigints };

use rustfft::{FftPlanner, num_complex::Complex};



#[test]
pub fn test_rustfft() {
    let num_inputs = 25600;
    let mut inputs = vec![Complex{ re: 0.0f32, im: 0.0f32 }; num_inputs];

    // Calculate FFT on the CPU
    let sw = Stopwatch::start_new();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(num_inputs);
    fft.process(&mut inputs);
    println!("CPU took {}ms", sw.elapsed_ms());

    // Now, `inputs` contains the FFT result
    for i in 0..10 {
        println!("FFT output [{}]: {:?}", i, inputs[i]);
    }

    // // Convert inputs to bytes
    // let input_to_gpu = complex_vec_to_bytes(inputs);

    // // Send to the GPU
    // let wgsl = concat_files(vec![
    //     "src/wgsl/fft.wgsl",
    // ]);

    // let sw = Stopwatch::start_new();
    // let result = pollster::block_on(single_buffer_compute(&wgsl, &input_to_gpu, num_inputs)).unwrap();
    // println!("GPU took {}ms", sw.elapsed_ms());

    // let result = bytes_to_complex(result);

    // for i in 0..num_inputs {
    //     assert_eq!(result[i], expected[i]);
    // }
}

#[test]
pub fn test_pow_5() {
    // The BN254 scalar field modulus
    let p = get_fr();

    let num_inputs = 256;
    let mut inputs = Vec::with_capacity(num_inputs);
    let mut expected = Vec::with_capacity(num_inputs);

    for _ in 0..num_inputs {
        // Generate a random field element
        let mut rng = rand::thread_rng();
        let random_bytes = rng.gen::<[u8; 32]>();
        let a = BigUint::from_bytes_be(random_bytes.as_slice()) % &p;

        inputs.push(a);
    }

    let sw = Stopwatch::start_new();
    for i in 0..num_inputs {
        let a = inputs[i].clone();
        let a_pow_5 = a.pow(5) % &p;
        expected.push(a_pow_5);
    }
    println!("CPU took {}ms", sw.elapsed_ms());

    let input_to_gpu = bigints_to_bytes(inputs);

    // Send to the GPU
    let wgsl = concat_files(
        vec![
            "src/wgsl/structs.wgsl",
            "src/wgsl/storage.wgsl",
            "src/wgsl/bigint.wgsl",
            "src/wgsl/fr.wgsl",
            "src/wgsl/pow_5.wgsl",
        ]
    );

    let sw = Stopwatch::start_new();
    let result = pollster::block_on(single_buffer_compute(&wgsl, &input_to_gpu, num_inputs)).unwrap();
    println!("GPU took {}ms", sw.elapsed_ms());

    let result = u32s_to_bigints(result);

    for i in 0..num_inputs {
        assert_eq!(result[i], expected[i]);
    }
}

#[test]
pub fn test_multi_pow_5() {
    // The BN254 scalar field modulus
    let p = get_fr();

    let num_inputs = 256;
    let mut inputs = Vec::with_capacity(num_inputs);
    let mut expected = Vec::with_capacity(num_inputs);

    for _ in 0..num_inputs {
        // Generate a random field element
        let mut rng = rand::thread_rng();
        let random_bytes = rng.gen::<[u8; 32]>();
        let a = BigUint::from_bytes_be(random_bytes.as_slice()) % &p;

        inputs.push(a);
    }

    let times_to_pow = 512;

    let sw = Stopwatch::start_new();
    for i in 0..num_inputs {
        let a = inputs[i].clone();
        let mut a_pow_5 = a;
        for _ in 0..times_to_pow {
            a_pow_5 = a_pow_5.pow(5) % &p;
        }

        expected.push(a_pow_5);
    }
    println!("CPU took {}ms", sw.elapsed_ms());

    let input_to_gpu = bigints_to_bytes(inputs);

    // Send to the GPU
    let wgsl = concat_files(
        vec![
            "src/wgsl/structs.wgsl",
            "src/wgsl/storage.wgsl",
            "src/wgsl/bigint.wgsl",
            "src/wgsl/fr.wgsl",
            "src/wgsl/multi_pow_5.wgsl",
        ]
    );

    let sw = Stopwatch::start_new();
    //let result = pollster::block_on(single_buffer_compute(&wgsl, &input_to_gpu, num_inputs)).unwrap();
    let result = pollster::block_on(single_buffer_compute(&wgsl, &input_to_gpu, 1)).unwrap();
    println!("GPU took {}ms", sw.elapsed_ms());

    let result = u32s_to_bigints(result);

    for i in 0..num_inputs {
        assert_eq!(result[i], expected[i]);
    }
}
