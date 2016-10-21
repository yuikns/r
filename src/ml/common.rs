//!
//! This package contains some common structs and functions
//!

pub struct DataSet<X, Y> (Vec<(Vec<X>, Y)>);

impl<X, Y> DataSet<X, Y> {
    /// init a new dataset
    pub fn new() -> DataSet<X, Y> {
        DataSet(Vec::new())
    }

    #[inline]
    pub fn ds(&mut self) -> &Vec<(Vec<X>, Y)> {
        &self.0
    }

    pub fn at(&mut self, i: usize) -> &(Vec<X>, Y) {
        &self.0[i]
    }

    pub fn x_at(&mut self, i: usize) -> &Vec<X> {
        &self.0[i].0
    }

    pub fn y_at(&mut self, i: usize) -> &Y {
        &self.0[i].1
    }
}