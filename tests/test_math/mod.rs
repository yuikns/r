
use r::math;

#[test]
fn abs() {
    assert!(math::abs(-1) == 1);
    assert!(math::abs(0) == 0);
    assert!(math::abs(1) == 1);
    assert!(math::abs(-1024000) == 1024000);
}