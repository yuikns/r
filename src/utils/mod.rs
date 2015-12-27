//!
//! Here are some toy functions, they will grown up.
//!
//!

///
/// This function will format your input string and return a greeting
///
///
pub fn greeting(s: &str) -> String{
    let rt = format!("it works! a voice comes from r::utils::greeting and in reply to {}", s);
    println!("{}",rt);
    rt
}