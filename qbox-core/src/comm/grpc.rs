use futures::Stream;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::task::{Context, Poll};
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod pb {
    tonic::include_proto!("qbox.api.grpc");
}

use pb::qbox_server::Qbox;
use pb::{QboxEvent, QboxRequest, QboxResponse, Topic, Void};

pub struct QboxServer;

#[tonic::async_trait]
impl Qbox for QboxServer {
    async fn call(&self, request: Request<QboxRequest>) -> Result<Response<QboxResponse>, Status> {
        unimplemented!()
    }
    async fn send(&self, request: Request<QboxRequest>) -> Result<Response<Void>, Status> {
        unimplemented!()
    }
    #[doc = "Server streaming response type for the subscribe method."]
    type SubscribeStream = mpsc::Receiver<Result<QboxEvent, Status>>;
    async fn subscribe(
        &self,
        request: Request<Topic>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for _ in 0..4 {
                tx.send(Ok(QboxEvent {
                    body: b"aa".to_vec(),
                }))
                .await;
            }
        });
        Ok(Response::new(rx))
    }
}
