use crate::counter::Bar;
use ta::indicators::Maximum as max;
use ta::Next;

pub trait MAX {
    fn max(&self, period: usize) -> f64;
}

impl MAX for Vec<f64> {
    fn max(&self, period: usize) -> f64 {
        let mut max = max::new(period).unwrap();
        self.iter()
            .map(|bar| max.next(*bar))
            .fold(0. / 0., f64::max)
    }
}

impl MAX for Vec<Bar> {
    fn max(&self, period: usize) -> f64 {
        let mut max = max::new(period).unwrap();
        self.iter().map(|bar| max.next(bar)).fold(0. / 0., f64::max)
    }
}
