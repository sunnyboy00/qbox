use crate::core::events;
use crate::core::topics::*;
use crossbeam::channel::{self, Receiver, Sender, TryRecvError};
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
#[cfg(unix)]
use tokio::net::UnixListener;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod pb {
    tonic::include_proto!("qbox.api.grpc");
}

use crate::bus::Token;
use crate::core::Event;
use ahash::RandomState;
use dashmap::DashMap;
use pb::qbox_server::Qbox;
use pb::{QboxRequest, QboxResponse, QboxStreamEvent, SubscribeRequest, Void};

pub struct QboxServer {
    tokens: DashMap<String, DashMap<String, Token, RandomState>, RandomState>,
    // tx: Sender<Result<QboxStreamEvent, Status>>,
    // rx: Receiver<Result<QboxStreamEvent, Status>>,
}

impl QboxServer {
    pub fn new() -> Self {
        // let (tx, rx) = channel::bounded(1024);
        Self {
            tokens: DashMap::with_hasher(RandomState::new()),
            // tx,
            // rx,
        }
    }
}

#[tonic::async_trait]
impl Qbox for QboxServer {
    async fn call(&self, request: Request<QboxRequest>) -> Result<Response<QboxResponse>, Status> {
        match ron::de::from_bytes::<Event>(&request.get_ref().body[..]) {
            Ok(ev) => match events::call(&request.get_ref().topic, ev) {
                Ok(ret) => match bincode::serialize(ret.as_ref()) {
                    Ok(b) => Ok(Response::new(QboxResponse {
                        topic: request.get_ref().topic.clone(),
                        body: b,
                    })),
                    Err(err) => {
                        return Err(Status::internal(format!("call error: {}", err)));
                    }
                },
                Err(err) => {
                    return Err(Status::invalid_argument(format!("call error: {}", err)));
                }
            },
            Err(err) => return Err(Status::invalid_argument(format!("event error: {}", err))),
        }
    }

    async fn send(&self, request: Request<QboxRequest>) -> Result<Response<Void>, Status> {
        match ron::de::from_bytes::<Event>(&request.get_ref().body[..]) {
            Ok(ev) => {
                if let Err(err) = crate::core::events::publish(&request.get_ref().topic, ev) {
                    return Err(Status::invalid_argument(format!("send error: {}", err)));
                }
            }
            Err(err) => return Err(Status::invalid_argument(format!("event error: {}", err))),
        }
        Ok(Response::new(Void {}))
    }

    #[doc = "订阅服务器事件"]
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<QboxStreamEvent, Status>> + Send>>; //mpsc::Receiver<Result<QboxEvent, Status>>;
    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        if let Some(client_id) = request.metadata().get("client_id") {
            let client_id = client_id.to_str().unwrap();
            if !self.tokens.contains_key(client_id) {
                self.tokens
                    .insert(client_id.into(), DashMap::with_hasher(RandomState::new()));
            }
            let topics = request.get_ref().topics;
            let (tx, rx) = channel::bounded(1024);
            let mut subs = self.tokens.get_mut(client_id).unwrap();
            for topic in topics.iter() {
                if !subs.contains_key(&topic) {
                    let tx = tx.clone();
                    match events::subscribe(&topic, move |topic, ev| {
                        match bincode::serialize(ev.as_ref()) {
                            Ok(b) => {
                                if let Err(err) = tx.try_send(Ok(QboxStreamEvent {
                                    topic: topic.into(),
                                    body: b,
                                })) {
                                    log::error!("tx error {}", err);
                                }
                            }
                            Err(err) => {
                                log::error!("bincode::serialize error {}", err);
                            }
                        }
                    }) {
                        Ok(token) => {
                            subs.insert(topic.to_string(), token);
                        }
                        Err(err) => {
                            return Err(Status::invalid_argument(format!(
                                "subscribe error: {}",
                                err
                            )))
                        }
                    }
                }
            }
            Ok(Response::new(
                Box::pin(EvStream(rx)) as Self::SubscribeStream
            ))
        } else {
            return Err(Status::invalid_argument("client_id is required"));
        }
    }
}

struct EvStream(Receiver<Result<QboxStreamEvent, Status>>);
impl Stream for EvStream {
    type Item = Result<QboxStreamEvent, Status>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let rx = &mut self.0;
        match rx.try_recv() {
            Ok(ev) => Poll::Ready(Some(ev)),
            Err(TryRecvError::Disconnected) => Poll::Ready(None),
            Err(TryRecvError::Empty) => Poll::Pending,
        }
    }
}
