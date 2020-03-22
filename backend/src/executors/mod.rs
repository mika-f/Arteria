use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::{web::Bytes, Error};
use futures::Stream;
use serde::Serialize;
use tokio::sync::mpsc::Receiver;

mod perl;
pub use perl::*;

#[derive(Clone, Debug, Serialize)]
pub enum Event {
  System,
  Heartbeat,
  Message,
  Command,
}

#[derive(Clone, Debug, Serialize)]
pub struct ExecutorEvent<T: Serialize> {
  pub event: Event,
  pub data: Option<T>,
}

pub struct HeartbeatEvent();

impl HeartbeatEvent {
  pub fn create() -> ExecutorEvent<()> {
    ExecutorEvent::<()> {
      event: Event::Heartbeat,
      data: None,
    }
  }
}

pub struct Client(Receiver<Bytes>);

impl Stream for Client {
  type Item = Result<Bytes, Error>;

  fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    match Pin::new(&mut self.0).poll_next(ctx) {
      Poll::Ready(Some(value)) => Poll::Ready(Some(Ok(value))),
      Poll::Ready(None) => Poll::Ready(None),
      Poll::Pending => Poll::Pending,
    }
  }
}

pub fn to_bytes<T: Serialize>(event: ExecutorEvent<T>) -> Bytes {
  let json = serde_json::to_string(&event).unwrap();

  Bytes::from(format!("data: {}\n\n", json))
}
