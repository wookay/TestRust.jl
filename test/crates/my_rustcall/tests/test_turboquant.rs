// from turboquant-rs/tests/test_codebook.rs

#[test]
fn lloyd_max_convergence_1bit() {
    use turboquant::{
            lloyd_max,
            codebook::{
                Codebook
            }
        };

    let d: usize = 64;
    let bit_width: u8 = 1;
    let max_iter: usize = 1000;
    let cb: Codebook = lloyd_max::solve(d, bit_width, max_iter);
    assert_eq!(cb.bit_width, 1);
    assert_eq!(cb.centroids.len(), 2);
    assert_eq!(cb.boundaries.len(), 1);
}


fn type_of<T>(_: T) -> String {
    return std::any::type_name::<T>().to_string();
}

// from turboquant-rs/tests/test_baseline.rs
#[test]
fn quantize_dequantize_roundtrip() {
    use turboquant::baseline::{
            quantize,
            dequantize,
            // UniformQuantized,
        };

    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let bit_width: u8 = 4;
    let q = quantize(&x, bit_width); // UniformQuantized
    assert_eq!(type_of(q.clone()), "turboquant::baseline::UniformQuantized");
    assert_eq!(type_of(&q), "&turboquant::baseline::UniformQuantized");
    dbg!(&q);
    let x_hat = dequantize(&q);
    dbg!(x_hat);
}
