use anyhow::{anyhow, Result};

pub(crate) fn get_with_default<T: std::str::FromStr>(key: &str, def: &str) -> Result<T, T::Err> {
    if let Ok(v) = dotenv::var(key) {
        if v == "" {
            def.to_owned().parse::<T>()
        } else {
            v.parse::<T>()
        }
    } else {
        def.to_owned().parse::<T>()
    }
}

pub(crate) fn get<T: std::str::FromStr>(key: &str) -> Result<T> {
    let t = dotenv::var(key)?;
    match t.parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => Err(anyhow!("parser error")),
    }
}