use anyhow::Result;

pub trait Filter<T>: Send + Sync {
    fn name(&self) -> &'static str {
        "Filter"
    }

    fn do_filter(&self, event: &T) -> Result<()>;

    fn chain<R: Filter<T>>(self, next: R) -> Chain<Self, R>
    where
        Self: Sized,
    {
        Chain {
            first: self,
            second: next,
        }
    }
}

#[derive(Default, Clone)]
pub struct Chain<W, U> {
    first: W,
    second: U,
}

impl<W, U> Chain<W, U> {
    pub fn into_inner(self) -> (W, U) {
        (self.first, self.second)
    }

    pub fn get_ref(&self) -> (&W, &U) {
        (&self.first, &self.second)
    }
    pub fn get_mut(&mut self) -> (&mut W, &mut U) {
        (&mut self.first, &mut self.second)
    }
}

impl<T, W: Filter<T>, U: Filter<T>> Filter<T> for Chain<W, U> {
    fn name(&self) -> &'static str {
        "FilterChain"
    }

    fn do_filter(&self, req: &T) -> Result<()> {
        self.first.do_filter(req)?;
        self.second.do_filter(req)?;
        Ok(())
    }
}
