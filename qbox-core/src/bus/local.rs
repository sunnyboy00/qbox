use super::{EventBus, Token};
use ahash::RandomState;
use anyhow::Result;
use dashmap::DashMap;
use rayon::prelude::*;
use std::any::Any;
pub struct LocalBus<T> {
    subscriber: DashMap<String, Vec<(Token, Box<dyn Fn(&str, T) + Send + Sync>)>, RandomState>,
    //filters: DashMap<Topic, Box<dyn Filter<T>>, RandomState>,
}

impl<T> LocalBus<T> {
    pub fn new() -> Self {
        let subscriber = DashMap::with_hasher(RandomState::new());
        // let filters = DashMap::with_hasher(RandomState::new());
        Self {
            subscriber,
            //filters,
        }
    }
}

impl<T: Send + Sync + Clone> EventBus for LocalBus<T> {
    type Message = T;
    fn publish<TOPIC: AsRef<str>>(&self, topic: TOPIC, msg: T) -> Result<()> {
        let topic = topic.as_ref();
        self.subscriber.iter().for_each(|item| {
            let key = item.key();
            let list = item.value();
            if topic == key || topic.starts_with(key) {
                list.par_iter().for_each(|(_, f)| f(topic, msg.clone()));
            }
        });
        Ok(())
    }

    fn subscribe<TOPIC: AsRef<str>>(
        &self,
        topic: TOPIC,
        f: impl Fn(&str, T) + Send + Sync + 'static,
    ) -> Result<Token> {
        let topic = topic.as_ref();
        let tid = f.type_id();
        let token = Token {
            topic: topic.into(),
            id: format!("{:?}", tid),
        };
        if let Some(mut list) = self.subscriber.get_mut(topic) {
            list.push((token.clone(), Box::new(f)));
        } else {
            let bf = Box::new(f) as Box<dyn Fn(&str, T) + Send + Sync>;
            let list = vec![(token.clone(), bf)];
            self.subscriber.insert(topic.into(), list);
        }

        Ok(token)
    }

    fn unsubscribe(&self, token: &Token) {
        if let Some(mut list) = self.subscriber.get_mut(token.topic.as_str()) {
            if let Some(idx) = list.iter().position(|(item, _)| item.id == token.id) {
                let _ = list.remove(idx);
            }
        }
    }

    // fn with_filter(&self, topic: Topic, filter: impl Filter<Self::Message> + 'static) {
    //     self.filters.insert(topic, Box::new(filter));
    // }
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
