//#[warn(unused_imports)]
//#[allow(unused_imports)]
use ml::common::DataSet;

pub struct Linear {
    ds: DataSet<f64, f64>,
}


impl Linear {
    pub fn new() -> Linear {
        Linear {
            ds: DataSet::new(),
        }
    }

    #[inline]
    pub fn ds_val(&mut self) -> &Vec<(Vec<f64>, f64)> {
        self.ds.ds()
    }

    #[inline]
    pub fn ds_x_at(&mut self, i: usize) -> &Vec<f64> {
        self.ds.x_at(i)
    }

    #[inline]
    pub fn ds_y_at(&mut self, i: usize) -> &f64 {
        self.ds.y_at(i)
    }
}