// This code is adapted from https://github.com/sampritipanda/msm-webgpu/blob/main/bigint.wgsl

fn fun(a: ptr<function, BigInt256>) -> BigInt256 {
    return fr_mul(a, a);
}

@compute
@workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var a: BigInt256 = buf[global_id.x];
    var times_to_pow = 512u;
    for (var i = 0u; i < times_to_pow; i++) {
        a = fun(&a);
    }
    buf[global_id.x] = a;
}
