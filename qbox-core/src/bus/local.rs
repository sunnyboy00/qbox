use super::Filter;
use super::{EventBus, Token, Topic};
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use std::any::Any;

pub struct LocalBus<T> {
    subscriber: DashMap<Topic, Vec<(Token, Box<dyn Fn(Topic, T) + Send + Sync>)>, RandomState>,
    filters: DashMap<Topic, Box<dyn Filter<T>>, RandomState>,
}

impl<T> LocalBus<T> {
    pub fn new() -> Self {
        let subscriber = DashMap::with_hasher(RandomState::new());
        let filters = DashMap::with_hasher(RandomState::new());
        Self {
            subscriber,
            filters,
        }
    }
}

impl<T: Send + Sync + Clone> EventBus for LocalBus<T> {
    type Message = T;
    fn publish(&self, topic: Topic, msg: T) -> Result<()> {
        if let Some(list) = self.subscriber.get(topic) {
            if let Some(filter) = self.filters.get(topic) {
                filter.do_filter(&msg)?;
            }
            // list.par_iter().for_each(|(_, f)| f(topic, msg.clone()));
            list.iter().for_each(|(_, f)| f(topic, msg.clone()));
        }
        Ok(())
    }

    fn subscribe(
        &self,
        topic: Topic,
        f: impl Fn(Topic, T) + Send + Sync + 'static,
    ) -> Result<Token> {
        let tid = f.type_id();
        let token = Token {
            topic,
            id: format!("{:?}", tid),
        };
        if let Some(mut list) = self.subscriber.get_mut(topic) {
            list.push((token.clone(), Box::new(f)));
        } else {
            let bf = Box::new(f) as Box<dyn Fn(Topic, T) + Send + Sync>;
            let list = vec![(token.clone(), bf)];
            self.subscriber.insert(topic, list);
        }
        Ok(token)
    }

    fn unsubscribe(&self, token: &Token) {
        if let Some(mut list) = self.subscriber.get_mut(token.topic) {
            if let Some(idx) = list.iter().position(|(item, _)| item.id == token.id) {
                let _ = list.remove(idx);
            }
        }
    }

    fn with_filter(&self, topic: Topic, filter: impl Filter<Self::Message> + 'static) {
        self.filters.insert(topic, Box::new(filter));
    }
}

mod tests {
    use super::super::EventBus;
    use super::LocalBus;
    use std::time::Duration;
    use url::Url;

    #[derive(Debug, Clone)]
    enum Message {
        Offer(String),
        Trade(String),
        Quote(String),
    }

    #[test]
    fn test() {
        let bus = LocalBus::<Message>::new();

        let token = bus
            .subscribe("/ctp/abc", |topic, ev| {
                println!("on {:?} {:?} {:?}", std::thread::current().id(), topic, ev)
            })
            .unwrap();
        println!("subscribe {:?}", token);
        let token = bus
            .subscribe("/ctp/abc", |topic, ev| {
                println!("on {:?} {:?} {:?}", std::thread::current().id(), topic, ev)
            })
            .unwrap();
        println!("subscribe {:?}", token);

        std::thread::spawn(move || {
            let mut n = 0;

            loop {
                bus.publish("/ctp/abc", Message::Offer("".to_string())).ok();
                std::thread::sleep(Duration::from_millis(1000));
                n += 1;
                if n == 10 {
                    bus.unsubscribe(&token);
                }
            }
        })
        .join()
        .ok();
    }
}
