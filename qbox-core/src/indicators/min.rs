use crate::broker::Bar;
use ta::indicators::Minimum as min;
use ta::Next;

pub trait MIN {
    fn min(&self, period: usize) -> f64;
}
impl MIN for Vec<f64> {
    fn min(&self, period: usize) -> f64 {
        let mut min = min::new(period).unwrap();
        self.iter()
            .map(|bar| min.next(*bar))
            .fold(0. / 0., f64::min)
    }
}

impl MIN for Vec<Bar> {
    fn min(&self, period: usize) -> f64 {
        let mut min = min::new(period).unwrap();
        self.iter().map(|bar| min.next(bar)).fold(0. / 0., f64::min)
    }
}
