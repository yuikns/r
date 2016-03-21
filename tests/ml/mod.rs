//!
//! This function check some kinds of Evaluation
//!
#[test]
fn eval() {
    use r::ml::eval::Eval;
    let mut e = Eval::new();
    e.add(true,true);
    e.add(false,false);
    e.add(true,false);
    e.add(true,false);
    e.add(false,true);
    assert!(e.accuracy() == 0.4f64);
    
    assert!(e.precision() > 0.333f64);
    assert!(e.precision() < 0.334f64);

    assert!(e.recall() == 0.5f64);
    
    assert!(e.f1() >= 0.3000f64);
    assert!(e.f1() <= 0.3001f64);
    
    assert!(e.f1() == e.f(1));
    assert!(e.f(2) == 0.25f64);
    
    e.reset();
    
    assert!(e.precision() == 0f64);
    assert!(e.recall() == 0f64);
    assert!(e.f1() == 0f64);
    assert!(e.f(1) == 0f64);
}

