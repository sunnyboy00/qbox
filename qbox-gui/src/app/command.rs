use druid::Selector;

use crate::trade::Instrument;

pub const QBOX_KLINE_LOSE_HOT: Selector = Selector::new("qbox-app.kline-lose-hot");
pub const QBOX_INSTRUMENT_CLICKED: Selector<Instrument> =
    Selector::new("qbox-app.instrument-clicked");

pub const QBOX_INSTRUMENT_SELECTED: Selector<Instrument> =
    Selector::new("qbox-app.instrument-selected");
pub const QBOX_INSTRUMENT_UNSELECTED: Selector<Instrument> =
    Selector::new("qbox-app.instrument-unselected");
