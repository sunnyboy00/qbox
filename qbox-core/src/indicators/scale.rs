use crate::counter::Bar;

pub trait Scale {
    type Output;
    fn scale(&self, s: f64) -> Self::Output;
}

impl Scale for Vec<Bar> {
    type Output = Vec<Bar>;
    fn scale(&self, s: f64) -> Vec<Bar> {
        let bars: Vec<Bar> = self
            .iter()
            .map(|bar| {
                let mut nbar = bar.clone();
                nbar.open = nbar.open * s;
                nbar.high = nbar.high * s;
                nbar.low = nbar.low * s;
                nbar.close = nbar.close * s;
                nbar.volume = nbar.volume * s;
                if let Some(turnover) = nbar.turnover {
                    nbar.turnover = Some(turnover * s);
                }
                nbar
            })
            .collect();
        bars
    }
}
impl Scale for Vec<(f64, f64, f64, f64)> {
    type Output = Vec<(f64, f64, f64, f64)>;
    fn scale(&self, s: f64) -> Vec<(f64, f64, f64, f64)> {
        let bars: Vec<(f64, f64, f64, f64)> = self
            .iter()
            .map(|(x1, x2, x3, x4)| (x1 * s, x2 * s, x3 * s, x4 * s))
            .collect();
        bars
    }
}

impl Scale for Vec<(f64, f64, f64)> {
    type Output = Vec<(f64, f64, f64)>;
    fn scale(&self, s: f64) -> Vec<(f64, f64, f64)> {
        let bars: Vec<(f64, f64, f64)> = self
            .iter()
            .map(|(x1, x2, x3)| (x1 * s, x2 * s, x3 * s))
            .collect();
        bars
    }
}

impl Scale for Vec<(f64, f64)> {
    type Output = Vec<(f64, f64)>;
    fn scale(&self, s: f64) -> Vec<(f64, f64)> {
        let bars: Vec<(f64, f64)> = self.iter().map(|(x1, x2)| (x1 * s, x2 * s)).collect();
        bars
    }
}

impl Scale for Vec<f64> {
    type Output = Vec<f64>;
    fn scale(&self, s: f64) -> Vec<f64> {
        let bars: Vec<f64> = self.iter().map(|x1| x1 * s).collect();
        bars
    }
}
