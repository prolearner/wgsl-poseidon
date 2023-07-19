// This code i// This code is adapted from https://github.com/sampritipanda/msm-webgpu/blob/main/bigint.wgsl

fn pow_5(a: ptr<function, BigInt256>) -> BigInt256 {
    var a2: BigInt256 = fr_mul(a, a);
    var a4: BigInt256 = fr_mul(&a2, &a2);
    return fr_mul(&a4, a);
}

@compute
@workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var a: BigInt256 = buf[global_id.x];
    var times_to_pow = 512u;
    for (var i = 0u; i < times_to_pow; i++) {
        a = pow_5(&a);
    }
    buf[global_id.x] = a;
}


// struct Complex {
//   x: f32,
//   y: f32,
// };
// type ComplexArray = [Complex; 8];

// fn fft(input: ComplexArray; 8) -> ComplexArray; 8 {
  // let mut output = input;

  // for _ in 0..7 {
  //   let twiddle = 1.0;
  //   for i in 0..8 - 1 {
  //     let j = i + 1;
  //     let temp = output[i];
  //     output[i] = output[i] + twiddle * output[j];
  //     output[j] = temp - twiddle * output[j];
  //     twiddle *= -1.0;
  //   }
//   }

//   return output;
// }

// let input = [Complex(1.0, 0.0), Complex(0.0, 1.0), Complex(-1.0, 0.0), Complex(0.0, -1.0), Complex(1.0, 1.0), Complex(-1.0, 1.0), Complex(1.0, -1.0), Complex(-1.0, -1.0)];
// let output = fft(input);

// struct Complex {
//     real: u32,
//     imag: u32,
// };

// struct ComplexArray {
//     data: array<Complex, 1024>, // Adjust size to your needs
// };

// fn complex_mul(a: Complex, b: Complex, p: u32) -> Complex {
//     return Complex((a.real * b.real + a.imag * b.imag) % p,
//                    (a.real * b.imag + a.imag * b.real) % p);
// }

// fn fft(a: ptr<function, ComplexArray>, p: u32, r: u32) {

//     for (var k: u32 = 0u; k < half_N; k = k + 1u) {
//         let even = (*a).data[2u*k];
//         let odd = (*a).data[2u*k + 1u];

//         let t = complex_mul(
//             odd,
//             Complex((pow(r, k * 2u) % p), 0u)
//         );

//         (*a).data[k] = Complex((even.real + t.real) % p, (even.imag + t.imag) % p);

//         (*a).data[k + half_N] = Complex((even.real - t.real + p) % p, (even.imag - t.imag + p) % p);
//     }
// }
