use rand::Rng;
use ark_bn254::Fr;
use stopwatch::Stopwatch;
use num_bigint::BigUint;
//use std::str::FromStr;
use crate::bn254::get_fr;
use crate::gpu::double_buffer_compute;
use crate::wgsl::concat_files;
use crate::utils::{ bigints_to_bytes, u32s_to_bigints };
use poseidon_ark::{ Poseidon, load_constants };

pub fn n_rounds(t: usize) -> (usize, usize) {
    let n_rounds_f: usize = 8;
    let n_rounds_p: Vec<usize> = vec![
        56, 57, 56, 60, 60, 63, 64, 63, 60, 66, 60, 65, 70, 60, 64, 68,
    ];

    (n_rounds_f, n_rounds_p[t - 2])
}

pub fn load_constants_c(num_inputs: usize) -> Vec<Fr> {
    let (n_rounds_f, n_rounds_p) = n_rounds(num_inputs + 1);
    let t = num_inputs + 1;
    let num_constants = (n_rounds_f + n_rounds_p) * 2;
    let mut constants_c_to_use = Vec::with_capacity(num_constants);

    let constants = load_constants();

    for i in 0..num_constants {
        constants_c_to_use.push(constants.c[t - 2][i]);
    }
    constants_c_to_use
}

pub fn fr_vec_to_biguint_vec(vals: &Vec<Fr>) -> Vec<BigUint> {
    vals.iter().map(|v| (*v).into()).collect()
}

#[test]
pub fn test_poseidon() {
    //let mut rng = rand::thread_rng();
    //let random_bytes = rng.gen::<[u8; 32]>();
    //let a = BigUint::from_bytes_be(random_bytes.as_slice()) % &p;
    //let a = BigUint::from_slice(&[1]);
    // Number of inputs: 1
    // t = 1 + 1 = 2

    let poseidon = Poseidon::new();
    let p_constants = load_constants();

    let num_inputs = 256 * 64;
    let num_x_workgroups = 256;

    println!("Computing {} Poseidon hashes in Rust / WebGPU", num_inputs);

    let mut inputs: Vec<BigUint> = Vec::with_capacity(num_inputs);

    let mut rng = rand::thread_rng();
    for _ in 0..num_inputs {
        let random_bytes = rng.gen::<[u8; 32]>();
        let a = BigUint::from_bytes_be(random_bytes.as_slice()) % get_fr();
        inputs.push(a);
    }

    let mut constants: Vec<BigUint> = Vec::with_capacity(p_constants.c.len() + 4);

    let t = 2;

    // Append the C constants
    for val in fr_vec_to_biguint_vec(&p_constants.c[t - 2]) {
        constants.push(val);
    }

    // Append the M constants
    for vec in &p_constants.m[t - 2] {
        for val in fr_vec_to_biguint_vec(&vec) {
            constants.push(val.clone());
        }
    }

    let sw = Stopwatch::start_new();
    let expected_hashes: Vec<BigUint> = inputs.iter().map(|a| poseidon.hash(vec![a.clone().into()]).unwrap().into()).collect();
    println!("CPU took {}ms", sw.elapsed_ms());

    //// For debugging:
    //let b0: Fr = Fr::from_str("0").unwrap();
    //let state = vec![b0.clone(), a.clone().into()];
    //let mut state = vec![b0.clone(), a.clone().into()];
    //let n_rounds_f = 8;
    //let n_rounds_p = 56;
    //for i in 0..(n_rounds_f + n_rounds_p) {
        //poseidon.ark(&mut state, &p_constants.c[t - 2], i * t);
        //poseidon.sbox(n_rounds_f, n_rounds_p, &mut state, i);
        //state = poseidon.mix(&state, &p_constants.m[t - 2]);
    //}
    //let expected_final_state = fr_vec_to_biguint_vec(&state);
    //println!("expected final state: {:?}", expected_final_state);

    //let input_to_gpu = bigints_to_bytes(fr_vec_to_biguint_vec(&inputs));
    let buf = bigints_to_bytes(inputs.clone());
    let constants = bigints_to_bytes(constants);

    // Passing the constants as hardcoded WGSL code is to inefficient
    //let wgsl = gen_poseidon_t2_wgsl();
    let wgsl = concat_files(
        vec![
            "src/wgsl/structs.wgsl",
            "src/wgsl/storage.wgsl",
            "src/wgsl/bigint.wgsl",
            "src/wgsl/fr.wgsl",
            "src/wgsl/poseidon_t2.wgsl"
        ]
    );

    //println!("{}", wgsl);

    // Send to the GPU
    let result = pollster::block_on(double_buffer_compute(&wgsl, &buf, &constants, num_x_workgroups, 1)).unwrap();

    let result = u32s_to_bigints(result);
    //println!("Input: {:?}", inputs.clone());
    //println!("Result from GPU: {:?}", result.clone());
    //assert_eq!(result[0], expected_final_state[0]);
    assert_eq!(result, expected_hashes);

}



#[test]
pub fn test_ntt() {
    //let mut rng = rand::thread_rng();
    //let random_bytes = rng.gen::<[u8; 32]>();
    //let a = BigUint::from_bytes_be(random_bytes.as_slice()) % &p;
    //let a = BigUint::from_slice(&[1]);
    // Number of inputs: 1
    // t = 1 + 1 = 2

    let poseidon = Poseidon::new();
    let p_constants = load_constants();

    let num_inputs = 256 * 64;
    let num_x_workgroups = 256;

    println!("Computing {} Poseidon hashes in Rust / WebGPU", num_inputs);

    let mut inputs: Vec<BigUint> = Vec::with_capacity(num_inputs);

    let mut rng = rand::thread_rng();
    for _ in 0..num_inputs {
        let random_bytes = rng.gen::<[u8; 32]>();
        let a = BigUint::from_bytes_be(random_bytes.as_slice()) % get_fr();
        inputs.push(a);
    }

    let mut constants: Vec<BigUint> = Vec::with_capacity(p_constants.c.len() + 4);

    let t = 2;

    // Append the C constants
    for val in fr_vec_to_biguint_vec(&p_constants.c[t - 2]) {
        constants.push(val);
    }

    // Append the M constants
    for vec in &p_constants.m[t - 2] {
        for val in fr_vec_to_biguint_vec(&vec) {
            constants.push(val.clone());
        }
    }

    let sw = Stopwatch::start_new();
    let expected_hashes: Vec<BigUint> = inputs.iter().map(|a| poseidon.hash(vec![a.clone().into()]).unwrap().into()).collect();
    println!("CPU took {}ms", sw.elapsed_ms());

    
    //// For debugging:
    //let b0: Fr = Fr::from_str("0").unwrap();
    //let state = vec![b0.clone(), a.clone().into()];
    //let mut state = vec![b0.clone(), a.clone().into()];
    //let n_rounds_f = 8;
    //let n_rounds_p = 56;
    //for i in 0..(n_rounds_f + n_rounds_p) {
        //poseidon.ark(&mut state, &p_constants.c[t - 2], i * t);
        //poseidon.sbox(n_rounds_f, n_rounds_p, &mut state, i);
        //state = poseidon.mix(&state, &p_constants.m[t - 2]);
    //}
    //let expected_final_state = fr_vec_to_biguint_vec(&state);
    //println!("expected final state: {:?}", expected_final_state);

    //let input_to_gpu = bigints_to_bytes(fr_vec_to_biguint_vec(&inputs));
    let buf = bigints_to_bytes(inputs.clone());
    let constants = bigints_to_bytes(constants);

    // Passing the constants as hardcoded WGSL code is to inefficient
    //let wgsl = gen_poseidon_t2_wgsl();
    let wgsl = concat_files(
        vec![
            "src/wgsl/structs.wgsl",
            "src/wgsl/storage.wgsl",
            "src/wgsl/bigint.wgsl",
            "src/wgsl/fr.wgsl",
            "src/wgsl/poseidon_t2.wgsl",
            "src/wgsl/ntt.wgsl"
        ]
    );

    //println!("{}", wgsl);

    // Send to the GPU
    let result = pollster::block_on(double_buffer_compute(&wgsl, &buf, &constants, num_x_workgroups, 1)).unwrap();

    let result = u32s_to_bigints(result);
    //println!("Input: {:?}", inputs.clone());
    //println!("Result from GPU: {:?}", result.clone());
    //assert_eq!(result[0], expected_final_state[0]);
    assert_eq!(result, expected_hashes);

}
