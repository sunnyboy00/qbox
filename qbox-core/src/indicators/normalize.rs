use crate::counter::Bar;

pub trait Normalize {
    type Output;
    fn normalize(&self) -> Self::Output;
}

impl Normalize for Vec<Bar> {
    type Output = Vec<Bar>;
    fn normalize(&self) -> Vec<Bar> {
        let ((max, min), (volume_max, volume_min), (turnover_max, turnover_min)) =
            self.iter().cloned().fold(
                ((0.0f64, 0.0f64), (0.0f64, 0.0f64), (0.0f64, 0.0f64)),
                |((max, min), (volume_max, volume_min), (turnover_max, turnover_min)), bar| {
                    (
                        (bar.high.max(max), bar.low.min(min)),
                        (bar.volume.max(volume_max), bar.volume.min(volume_min)),
                        (
                            if let Some(turnover) = bar.turnover {
                                turnover.max(turnover_max)
                            } else {
                                0.0
                            },
                            if let Some(turnover) = bar.turnover {
                                turnover.min(turnover_min)
                            } else {
                                0.0
                            },
                        ),
                    )
                },
            );
        let bars: Vec<Bar> = self
            .iter()
            .map(|bar| {
                let mut bar = bar.clone();
                bar.open = (bar.open - min) / (max - min);
                bar.close = (bar.close - min) / (max - min);
                bar.high = (bar.high - min) / (max - min);
                bar.low = (bar.low - min) / (max - min);
                bar.volume = (bar.volume - volume_min) / (volume_max - volume_min);
                bar.turnover = if let Some(turnover) = bar.turnover {
                    Some((turnover - turnover_min) / (turnover_max - turnover_min))
                } else {
                    None
                };
                bar
            })
            .collect();
        bars
    }
}

impl Normalize for Vec<(f64, f64, f64, f64)> {
    type Output = Vec<(f64, f64, f64, f64)>;
    fn normalize(&self) -> Vec<(f64, f64, f64, f64)> {
        let (max, min) =
            self.iter()
                .cloned()
                .fold((0.0f64, 0.0f64), |(max, min), (v1, v2, v3, v4)| {
                    (
                        max.max(v1).max(v2).max(v3).max(v4),
                        min.min(v1).min(v2).min(v3).min(v4),
                    )
                });
        let bars: Vec<(f64, f64, f64, f64)> = self
            .iter()
            .map(|(x1, x2, x3, x4)| {
                (
                    (x1 - min) / (max - min),
                    (x2 - min) / (max - min),
                    (x3 - min) / (max - min),
                    (x4 - min) / (max - min),
                )
            })
            .collect();
        bars
    }
}

impl Normalize for Vec<(f64, f64, f64)> {
    type Output = Vec<(f64, f64, f64)>;
    fn normalize(&self) -> Vec<(f64, f64, f64)> {
        let (max, min) = self
            .iter()
            .cloned()
            .fold((0.0f64, 0.0f64), |(max, min), (v1, v2, v3)| {
                (max.max(v1).max(v2).max(v3), min.min(v1).min(v2).min(v3))
            });
        let bars: Vec<(f64, f64, f64)> = self
            .iter()
            .map(|(x1, x2, x3)| {
                (
                    (x1 - min) / (max - min),
                    (x2 - min) / (max - min),
                    (x3 - min) / (max - min),
                )
            })
            .collect();
        bars
    }
}

impl Normalize for Vec<(f64, f64)> {
    type Output = Vec<(f64, f64)>;
    fn normalize(&self) -> Vec<(f64, f64)> {
        let (max, min) = self
            .iter()
            .cloned()
            .fold((0.0f64, 0.0f64), |(max, min), (v1, v2)| {
                (max.max(v1).max(v2), min.min(v1).min(v2))
            });
        let bars: Vec<(f64, f64)> = self
            .iter()
            .map(|(x1, x2)| ((x1 - min) / (max - min), (x2 - min) / (max - min)))
            .collect();
        bars
    }
}

impl Normalize for Vec<f64> {
    type Output = Vec<f64>;
    fn normalize(&self) -> Vec<f64> {
        let (max, min) = self
            .iter()
            .cloned()
            .fold((0.0f64, 0.0f64), |(max, min), v1| {
                (max.max(v1), min.min(v1))
            });
        let bars: Vec<f64> = self.iter().map(|x1| (x1 - min) / (max - min)).collect();
        bars
    }
}
