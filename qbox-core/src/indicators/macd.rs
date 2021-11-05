use crate::broker::Bar;
use ta::indicators::MovingAverageConvergenceDivergence as macd;
use ta::Next;

pub trait MACD {
    fn macd(&self, fast: usize, slow: usize, signal: usize) -> Vec<(f64, f64, f64)>;
}

impl MACD for Vec<f64> {
    fn macd(&self, fast: usize, slow: usize, signal: usize) -> Vec<(f64, f64, f64)> {
        let mut macd = macd::new(fast, slow, signal).unwrap();
        let ndata: Vec<(f64, f64, f64)> = self.iter().map(|bar| macd.next(*bar).into()).collect();
        ndata
    }
}

impl MACD for Vec<Bar> {
    fn macd(&self, fast: usize, slow: usize, signal: usize) -> Vec<(f64, f64, f64)> {
        let mut macd = macd::new(fast, slow, signal).unwrap();
        let ndata: Vec<(f64, f64, f64)> = self.iter().map(|bar| macd.next(bar).into()).collect();
        ndata
    }
}
