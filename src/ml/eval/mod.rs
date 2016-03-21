//!
//! Evaluation is a import progress in juding the result of Machine Learning
//! This structure contains some data to calculate Precision, Recall, Accuracy and F-Score
//!


use std::sync::atomic::{AtomicIsize, Ordering};

pub struct Eval {
    tp_sz: AtomicIsize, // true positives
    tn_sz: AtomicIsize, // true negatives 
    fp_sz: AtomicIsize, // false positives
    fn_sz: AtomicIsize, // false negatives
}

impl Eval {
    
    /// New Instance of Eval
    /// 
    /// # Example
    ///
    /// ```
    /// use r::ml::eval::Eval;
    /// let mut e = Eval::new();
    /// ```
    pub fn new() -> Eval {
        Eval { 
            tp_sz: AtomicIsize::new(0), 
            tn_sz: AtomicIsize::new(0), 
            fp_sz: AtomicIsize::new(0), 
            fn_sz: AtomicIsize::new(0), 
        }
    }

    /// Add A New Doc with Predicted Result and Actual Result
    pub fn add(&mut self, predicted:bool, actual:bool) {
        match predicted {
            true =>
                match actual {
                    true =>
                        self.tp_sz.fetch_add(1, Ordering::SeqCst),
                    false =>
                        self.fp_sz.fetch_add(1, Ordering::SeqCst),
                },
            false =>
                match actual {
                    true =>
                        self.fn_sz.fetch_add(1, Ordering::SeqCst),
                    false =>
                        self.tn_sz.fetch_add(1, Ordering::SeqCst),
                },
        };
    }

    /// Get Accuracy Result
    pub fn accuracy(&self) -> f64 {
        let denominator:f64 = (self.tp_sz.load(Ordering::SeqCst) + 
                                self.fp_sz.load(Ordering::SeqCst) + 
                                self.tn_sz.load(Ordering::SeqCst) + 
                                self.fn_sz.load(Ordering::SeqCst)) as f64;
        if denominator == 0f64 {
            0f64
        } else {
            let molecular:f64 = (self.tp_sz.load(Ordering::SeqCst) + self.tn_sz.load(Ordering::SeqCst)) as f64;
            molecular / denominator
        }
    }

    /// Get Precision Result
    pub fn precision(&self) -> f64 {
        if self.tp_sz.load(Ordering::SeqCst) + self.fp_sz.load(Ordering::SeqCst) == 0 {
            0.0f64
        } else {
            let molecular:f64 = self.tp_sz.load(Ordering::SeqCst) as f64;
            let denominator:f64 = (self.tp_sz.load(Ordering::SeqCst) + self.fp_sz.load(Ordering::SeqCst)) as f64;
            molecular / denominator
        }
    }
    
    /// Get Recall Result
    pub fn recall(&self) -> f64 {
        if self.tp_sz.load(Ordering::SeqCst) + self.fn_sz.load(Ordering::SeqCst) == 0 {
            0.0f64
        } else {
            let molecular:f64 = self.tp_sz.load(Ordering::SeqCst) as f64;
            let denominator:f64 = (self.tp_sz.load(Ordering::SeqCst) + self.fn_sz.load(Ordering::SeqCst)) as f64;
            molecular / denominator
        }
    }

    /// Get F1 Score
    pub fn f1(&self) -> f64 {
        self.f(1u64)
    }
    
    /// Get F_a Score
    pub fn f(&self, a:u64) -> f64 {
        if a == 0u64 {
            0.0f64
        } else {
            let p:f64 = self.precision();
            let r:f64 = self.recall();
            if p + r == 0f64 {
                0.0f64
            } else{
                let a2 = (a * 2) as f64;
                (a2 + 1.0f64) * p * r / (a2 * (p + r))
            }
        }
    }
    
    /// Reset All Counts
    pub fn reset(&mut self) {
        self.tp_sz.store(0,Ordering::Relaxed);
        self.tn_sz.store(0,Ordering::Relaxed);
        self.fp_sz.store(0,Ordering::Relaxed);
        self.fn_sz.store(0,Ordering::Relaxed);
    }
}
