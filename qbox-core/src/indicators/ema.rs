use crate::counter::Bar;
use ta::indicators::ExponentialMovingAverage as ema;
use ta::Next;

pub trait EMA {
    fn ema(&self, period: usize) -> Vec<f64>;
}

impl EMA for Vec<f64> {
    fn ema(&self, period: usize) -> Vec<f64> {
        let mut ema = ema::new(period).unwrap();
        let ndata: Vec<f64> = self.iter().map(|bar| ema.next(*bar)).collect();
        ndata
    }
}

impl EMA for Vec<Bar> {
    fn ema(&self, period: usize) -> Vec<f64> {
        let mut ema = ema::new(period).unwrap();
        let ndata: Vec<f64> = self.iter().map(|bar| ema.next(bar)).collect();
        ndata
    }
}
