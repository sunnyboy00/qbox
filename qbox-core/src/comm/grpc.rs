use crossbeam::channel::{self, Receiver, Sender, TryRecvError};
use futures::Stream;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
#[cfg(unix)]
use tokio::net::UnixListener;
// use tokio::sync::mpsc;
// use tokio::sync::mpsc::error::TryRecvError;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod pb {
    tonic::include_proto!("qbox.api.grpc");
}

use crate::bus::Token;
use crate::core::Event;
use ahash::RandomState;
use dashmap::DashMap;
use pb::qbox_server::Qbox;
use pb::{QboxRequest, QboxResponse, QboxStreamEvent, Void};

pub struct QboxServer {
    tokens: DashMap<String, Token, RandomState>,
    tx: Sender<Result<QboxStreamEvent, Status>>,
    rx: Receiver<Result<QboxStreamEvent, Status>>,
}

impl QboxServer {
    pub fn new() -> Self {
        let (tx, rx) = channel::bounded(1024);
        Self {
            tokens: DashMap::with_hasher(RandomState::new()),
            tx,
            rx,
        }
    }
}

#[tonic::async_trait]
impl Qbox for QboxServer {
    async fn call(&self, request: Request<QboxRequest>) -> Result<Response<QboxResponse>, Status> {
        unimplemented!()
    }
    async fn send(&self, request: Request<QboxRequest>) -> Result<Response<Void>, Status> {
        match ron::de::from_bytes::<Event>(&request.get_ref().body[..]) {
            Ok(ev) => {
                match ev {
                    Event::Startup => {}
                    Event::Shutdown => {}
                    Event::StartQuoter(uri) => {}
                    Event::StartTrader(uri) => {}
                    Event::StopQuoter(uri) => {}
                    Event::StopTrader(uri) => {}
                    Event::Trade(ev) => {}
                    Event::Quote(ev) => {}
                    _ => {}
                }
                // if let Err(err) = crate::core::events::publish(&request.get_ref().topic, ev) {
                //     return Err(Status::invalid_argument(format!("send error: {}", err)));
                // }
            }
            Err(err) => return Err(Status::invalid_argument(format!("event error: {}", err))),
        }
        Ok(Response::new(Void {}))
    }
    // async fn unsubscribe(&self, request: Request<Topic>) -> Result<Response<Void>, Status> {
    //     request.get_ref().topics.iter().for_each(|topic| {
    //         if let Some(token) = self.tokens.get(topic) {
    //             crate::core::events::unsubscribe(token.value());
    //         }
    //     });
    //     Ok(Response::new(Void {}))
    // }
    #[doc = "订阅服务器事件"]
    type EventsStream = Pin<Box<dyn Stream<Item = Result<QboxStreamEvent, Status>> + Send>>; //mpsc::Receiver<Result<QboxEvent, Status>>;
    async fn events(
        &self,
        _request: Request<Void>,
    ) -> Result<Response<Self::EventsStream>, Status> {
        // request.get_ref().topics.iter().for_each(|topic| {
        //     if !self.tokens.contains_key(topic) {
        //         let tx = tx.clone();
        //         if let Ok(token) =
        //             crate::core::events::subscribe(topic, move |_topic, ev: Arc<Event>| {
        //                 let b = bincode::serialize(ev.as_ref()).unwrap();
        //                 if let Err(err) = tx.try_send(Ok(QboxStreamEvent { body: b })) {
        //                     log::error!("{}", err);
        //                 }
        //             })
        //         {
        //             self.tokens.insert(topic.clone(), token);
        //         }
        //     }
        // });

        struct EvStream(Receiver<Result<QboxStreamEvent, Status>>);
        impl Stream for EvStream {
            type Item = Result<QboxStreamEvent, Status>;
            fn poll_next(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Option<Self::Item>> {
                let rx = &mut self.0;
                match rx.try_recv() {
                    Ok(ev) => Poll::Ready(Some(ev)),
                    Err(TryRecvError::Disconnected) => Poll::Ready(None),
                    Err(TryRecvError::Empty) => Poll::Pending,
                }
            }
        }
        Ok(Response::new(
            Box::pin(EvStream(self.rx.clone())) as Self::EventsStream
        ))
    }
}
