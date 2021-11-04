use crate::counter::Bar;
use ta::indicators::SimpleMovingAverage as ma;
use ta::Next;

pub trait SMA {
    fn ma(&self, period: usize) -> Vec<f64>;
}

impl SMA for Vec<f64> {
    fn ma(&self, period: usize) -> Vec<f64> {
        let mut ma = ma::new(period).unwrap();
        let ndata: Vec<f64> = self.iter().map(|bar| ma.next(*bar)).collect();
        ndata
    }
}

impl SMA for Vec<Bar> {
    fn ma(&self, period: usize) -> Vec<f64> {
        let mut ma = ma::new(period).unwrap();
        let ndata: Vec<f64> = self.iter().map(|bar| ma.next(bar)).collect();
        ndata
    }
}
