mod app;
mod opt;
mod ui;
mod util;

use anyhow::Result;
use flexi_logger::{FileSpec, Logger};
use opt::Opt;
use qbox_core::broker::*;
use qbox_core::{topics, Event};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use url::Url;

fn main() -> Result<()> {
    let opt = Opt::from_file(std::env::args().nth(2).take().unwrap().as_str())?;

    Logger::try_with_str(opt.level.as_str())?
        .format(flexi_logger::detailed_format)
        .log_to_file(
            FileSpec::default()
                .suppress_timestamp()
                .directory(Path::new(qbox_core::log_path()).join("logs"))
                .basename("qbox")
                .discriminant("qbox-cli")
                .suffix("log"),
        )
        .start()?;

    qbox_core::startup()?;
    qbox_broker::load_driver()?;

    let quoter = quoter::spawn(Url::parse(opt.quote_dsn.as_str())?)?;
    let trader = trader::spawn(Url::parse(opt.trade_dsn.as_str())?)?;
    trader.instruments(&[]);
    if let Some(instrs) = qbox_core::get_all_instrument() {
        let filter: Vec<_> = instrs
            .iter()
            .map(|instr| instr.security_id.clone())
            .collect();
        let filter: Vec<&str> = filter.iter().map(|sid| &**sid).collect();
        quoter.subscribe(&filter[..]);
    } else {
        qbox_core::subscribe(topics::QUERY_EVENT, move |_topic, ev| {
            if let Event::Trade(TradeEvent::InstrumentsResponse(instr)) = ev.as_ref() {
                quoter.subscribe(&[instr.security_id.as_str()]);
            }
        })?;
    }
    // loop {
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    // }
    app::run_app()
}
