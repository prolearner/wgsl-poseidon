//use ark_bn254::{G1Affine, G2Affine};
use rand::Rng;
use stopwatch::Stopwatch;
use num_bigint::BigUint;
use crate::bn254::get_fr;
use crate::wgsl::concat_files;
use crate::utils::{ bigints_to_bytes, u32s_to_bigints };

use ark_ff::{PrimeField, BigInteger};
use ark_bn254::Fr;
use ark_poly::{EvaluationDomain, GeneralEvaluationDomain};
//use ethers::types::U256;
use std::string::String;
use std::io::Write;
use crate::gpu::single_buffer_compute;

//pub fn u256_to_hex(val: U256) -> String {
    //let b: &mut [u8; 32] = &mut [0u8; 32];
    //val.to_big_endian(b);
    //hex::encode(&b).to_uppercase()
//}

//pub fn f_to_u256<F: PrimeField>(val: F) -> U256 {
    //let mut b = Vec::with_capacity(32);
    //let _ = val.write(&mut b);
    //let b_as_arr: [u8; 32] = b.try_into().unwrap();
    //U256::from_little_endian(&b_as_arr)
//}

pub fn f_to_hex<F: PrimeField>(val: F) -> String {
    hex::encode(val.into_bigint().to_bytes_be())
}

#[test]
pub fn test_fft() {
    // Fr is the BN254 scalar field
    let mut v = vec![Fr::from(1), Fr::from(2)];
    let domain = GeneralEvaluationDomain::<Fr>::new(v.len()).unwrap();
    domain.fft_in_place(&mut v);
    println!("{:?}", f_to_hex(v[0]));
    println!("{:?}", f_to_hex(v[1]));

    // minimal multiplication to test the env, hacked from mul_pow
    let p = get_fr();
    let num_inputs = 256;
    // let mut inputs = vec![Complex{ re: 0.0f32, im: 0.0f32 }; num_inputs];
    let mut inputs = Vec::with_capacity(num_inputs);
    for _ in 0..num_inputs {
        // Generate a random field element
        let mut rng = rand::thread_rng();
        let random_bytes = rng.gen::<[u8; 32]>();
        let a = BigUint::from_bytes_be(random_bytes.as_slice()) % &p;

        inputs.push(a);
    }

    // Convert inputs to bytes
    let input_to_gpu = bigints_to_bytes(inputs);
    // Send to the GPU
    let wgsl = concat_files(
        vec![
            "src/wgsl/structs.wgsl",
            "src/wgsl/storage.wgsl",
            "src/wgsl/bigint.wgsl",
            "src/wgsl/fr.wgsl",
            "src/wgsl/fft.wgsl"
        ]
    );

    let sw = Stopwatch::start_new();
    let result = pollster::block_on(single_buffer_compute(&wgsl, &input_to_gpu, num_inputs)).unwrap();
    println!("GPU took {}ms", sw.elapsed_ms());
}

