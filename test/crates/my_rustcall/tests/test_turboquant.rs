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
