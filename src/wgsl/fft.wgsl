struct Complex {
    real: u32,
    imag: u32,
};

struct ComplexArray {
    data: array<Complex, 1024>, // Adjust size to your needs
};

fn complex_mul(a: Complex, b: Complex, p: u32) -> Complex {
    return Complex((a.real * b.real + a.imag * b.imag) % p,
                   (a.real * b.imag + a.imag * b.real) % p);
}

fn fft(a: ptr<function, ComplexArray>, N: u32, p: u32, r: u32) {
    let half_N = N / 2;
    for (var k: u32 = 0u; k < half_N; k = k + 1u) {
        let even = (*a).data[2u*k];
        let odd = (*a).data[2u*k + 1u];

        let t = complex_mul(
            odd,
            Complex((pow(r, 2 * k) % p), 0)
        );

        (*a).data[k] = Complex((even.real + t.real) % p, (even.imag + t.imag) % p);

        (*a).data[k + half_N] = Complex((even.real - t.real + p) % p, (even.imag - t.imag + p) % p);
    }
}
