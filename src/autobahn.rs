use futures_util::{SinkExt, StreamExt};



use log::{debug, error, info, warn};

use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, accept_hdr_async, tungstenite::Error};
use tungstenite::Result;

use chrono::{DateTime, Utc};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
  if let Err(e) = handle_connection(peer, stream).await {
    match e {
      Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
      err => error!("Error processing connection: {}", err),
    }
  }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
  let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

  info!("New WebSocket connection: {}", peer);
  // let youMsgh = ws_stream.read_message().unwrap();
  while let Some(msg) = ws_stream.next().await {
    let msg = msg?;
    let msg_str = msg.to_text().unwrap();
    match msg_str {
      x if x.contains("[info]") == true => info!("{}", str::replace(msg_str, "[info]", "")),
      x if x.contains("[warn]") == true => warn!("{}", str::replace(msg_str, "[warn]", "")),
      x if x.contains("[error]") == true => error!("{}", str::replace(msg_str, "[error]", "")),
      x if x.contains("[debug]") == true => debug!("{}", str::replace(msg_str, "[debug]", "")),
      _ => info!("{}", msg_str),
    }
    if msg.is_text() || msg.is_binary() {
      ws_stream.send(msg).await?;
    }
  }

  Ok(())
}

#[tokio::main]
async fn main() {
  // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
  log_setup();

  let addr = "127.0.0.1:9002";
  let listener = TcpListener::bind(&addr).await.expect("Can't listen");
  info!("Listening on: {}", addr);

  while let Ok((stream, _)) = listener.accept().await {
    // connect accept
    let peer = stream
      .peer_addr()
      .expect("connected streams should have a peer address");
    info!("Peer address: {}", peer);

    tokio::spawn(accept_connection(peer, stream));
  }
}
