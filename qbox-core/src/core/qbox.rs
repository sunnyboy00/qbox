use super::events;
use anyhow::Result;

pub fn init() -> Result<()> {
    // events::path("/broker/trades/create", |topic, ev| {})?;
    // events::path("/broker/trades/suspend", |topic, ev| {})?;
    // events::path("/broker/trades/resume", |topic, ev| {})?;
    // events::path("/broker/trades/delete", |topic, ev| {})?;
    // events::path("/broker/trades/offer", |topic, ev| {})?;
    // events::path("/broker/trades/cancel", |topic, ev| {})?;
    // events::path("/broker/trades/list", |topic, ev| {})?;

    // events::path("/broker/quotes/create", |topic, ev| {})?;
    // events::path("/broker/quotes/suspend", |topic, ev| {})?;
    // events::path("/broker/quotes/resume", |topic, ev| {})?;
    // events::path("/broker/quotes/delete", |topic, ev| {})?;
    // events::path("/broker/quotes/subscribe", |topic, ev| {})?;
    // events::path("/broker/quotes/unsubscribe", |topic, ev| {})?;
    // events::path("/broker/quotes/list", |topic, ev| {})?;

    // events::path("/qbox/strategies/create", |topic, ev| {})?;
    // events::path("/qbox/strategies/suspend", |topic, ev| {})?;
    // events::path("/qbox/strategies/resume", |topic, ev| {})?;
    // events::path("/qbox/strategies/delete", |topic, ev| {})?;
    // events::path("/qbox/strategies/list", |topic, ev| {})?;
    Ok(())
}
