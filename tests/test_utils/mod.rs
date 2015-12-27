
use r::utils;

#[test]
fn greeting() {
    let rt = utils::greeting("abc");
    assert!(rt == "it works! a voice comes from r::utils::greeting and in reply to abc");
}
