// from turboquant-rs/tests/test_codebook.rs

use turboquant::lloyd_max::*;

#[test]
fn test_lloyd_max_convergence_1bit() {
    let cb = solve(64, 1, 1000);
    assert_eq!(cb.centroids.len(), 2);
}
