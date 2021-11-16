use crate::core::events;
use crossbeam::channel::{self, Receiver, TryRecvError, TrySendError};
use futures::Stream;
use lazy_static::lazy_static;
use std::pin::Pin;
use std::task::{Context, Poll};
#[cfg(unix)]
use tokio::net::UnixListener;
use tonic::{Request, Response, Status};
pub mod pb {
    tonic::include_proto!("qbox.api.grpc");
}

use crate::bus::Token;
use crate::core::Event;
use ahash::RandomState;
use dashmap::DashMap;
use pb::qbox_server::Qbox;
use pb::{QboxRequest, QboxResponse, QboxStreamEvent, SubscribeRequest, Void};

lazy_static! {
    static ref TOKENS: DashMap<String, DashMap<String, Token, RandomState>, RandomState> =
        DashMap::with_hasher(RandomState::new());
}
pub struct QboxServer;

#[tonic::async_trait]
impl Qbox for QboxServer {
    async fn call(&self, request: Request<QboxRequest>) -> Result<Response<QboxResponse>, Status> {
        match ron::de::from_bytes::<Event>(&request.get_ref().body[..]) {
            Ok(ev) => match events::call(&request.get_ref().path, ev) {
                Ok(ret) => match bincode::serialize(ret.as_ref()) {
                    Ok(b) => Ok(Response::new(QboxResponse {
                        path: request.get_ref().path.clone(),
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

    async fn send(&self, request: Request<QboxStreamEvent>) -> Result<Response<Void>, Status> {
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
            let client_id = client_id.to_str().unwrap().to_owned();
            if !TOKENS.contains_key(&client_id) {
                TOKENS.insert(client_id.clone(), DashMap::with_hasher(RandomState::new()));
            }
            let (tx, rx) = channel::bounded(1024);
            for topic in request.get_ref().topics.iter() {
                if !TOKENS.get(&client_id).unwrap().contains_key(topic) {
                    let tx = tx.clone();
                    let cid = client_id.clone();
                    match events::subscribe(topic, move |topic, ev| {
                        match bincode::serialize(ev.as_ref()) {
                            Ok(b) => {
                                match tx.try_send(Ok(QboxStreamEvent {
                                    topic: topic.into(),
                                    body: b,
                                })) {
                                    Ok(_) => {}
                                    Err(TrySendError::Disconnected(_)) => {
                                        if let Some((_, token)) =
                                            TOKENS.get(&cid).unwrap().remove(topic)
                                        {
                                            events::unsubscribe(&token);
                                        }
                                    }
                                    Err(err) => {
                                        log::error!("tx.try_send error {}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                log::error!("bincode::serialize error {}", err);
                            }
                        }
                    }) {
                        Ok(token) => {
                            TOKENS
                                .get(&client_id)
                                .unwrap()
                                .insert(topic.to_string(), token);
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
