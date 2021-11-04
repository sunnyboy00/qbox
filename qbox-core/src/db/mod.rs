mod assets;
mod orders;
mod positions;
mod quotes;

pub use assets::*;
pub use orders::*;
pub use positions::*;
pub use quotes::*;

use anyhow::Result;

pub fn startup() -> Result<()> {
    quotes::init()?;
    orders::init()?;
    positions::init()?;
    Ok(())
}
