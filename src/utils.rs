use num_bigint::BigUint;
use num_traits::identities::Zero;
use itertools::Itertools;
use rustfft::num_complex::Complex;

pub fn split_u32(a: u32) -> [u32; 2] {
    let a_0 = (a & 0xffff0000) >> 16;
    let a_1 = a & 0x0000ffff;
    [a_0, a_1]
}

pub fn split_biguint(a: BigUint) -> Vec<u8> {
    let mut a_bytes = a.to_bytes_le().to_vec();
    assert!(a_bytes.len() <= 32);

    while a_bytes.len() < 32 {
        a_bytes.push(0u8);
    }

    let mut result = Vec::with_capacity(64);
    let mut i = 0;
    loop {
        if i >= a_bytes.len() {
            break
        }

        result.push(a_bytes[i]);
        result.push(a_bytes[i + 1]);
        result.push(0u8);
        result.push(0u8);
        i += 2;
    }

    result
}

pub fn limbs_to_bigint256(limbs: &[u32]) -> BigUint {
    let mut res = BigUint::zero();
    for (i, limb) in limbs.iter().enumerate() {
        res += BigUint::from_slice(&[2]).pow((i * 16).try_into().unwrap()) * BigUint::from_slice(&[limb.clone()]);
    }

    res
}

pub fn bigint_to_limbs(p: &BigUint) -> Vec<u32> {
    let mut limbs: Vec<u32> = Vec::with_capacity(16);
    for c in split_biguint(p.clone()).into_iter().chunks(4).into_iter() {
        let bytes = c.collect::<Vec<u8>>();
        let limb: u32 = bytemuck::cast_slice(&bytes).to_vec()[0];
        limbs.push(limb);
    }

    limbs
}

pub fn bigints_to_bytes(vals: Vec<BigUint>) -> Vec<u8> {
    let mut input_as_bytes: Vec<Vec<u8>> = Vec::with_capacity(vals.len());
    for i in 0..vals.len() {
        input_as_bytes.push(split_biguint(vals[i].clone()));
    }

    input_as_bytes.into_iter().flatten().collect()
}


pub fn u32s_to_bigints(b: Vec<u32>) -> Vec<BigUint> { 
    let chunks: Vec<Vec<u32>> = b
        .into_iter().chunks(16)
        .into_iter().map(|c| c.into_iter().collect())
        .collect();

    chunks.iter().map(|c| limbs_to_bigint256(c)).collect()
}

pub fn complex_to_bytes(c: Complex<f32>) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    bytes[0..4].copy_from_slice(&c.re.to_bits().to_le_bytes());
    bytes[4..8].copy_from_slice(&c.im.to_bits().to_le_bytes());
    bytes
}

pub fn complex_vec_to_bytes(vec: Vec<Complex<f32>>) -> Vec<u8> {
    vec.iter().flat_map(|&c| complex_to_bytes(c).to_vec()).collect()
}

