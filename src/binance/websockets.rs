use std::sync::atomic::{AtomicBool, Ordering};

use futures::{SinkExt, StreamExt};
use log::error;
use serde_json::from_str;
use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use url::Url;

use crate::binance::config::Config;
use crate::binance::errors::*;

pub static STREAM_ENDPOINT: &str = "stream";
pub static WS_ENDPOINT: &str = "ws";
pub static OUTBOUND_ACCOUNT_INFO: &str = "outboundAccountInfo";
pub static OUTBOUND_ACCOUNT_POSITION: &str = "outboundAccountPosition";
pub static EXECUTION_REPORT: &str = "executionReport";
pub static KLINE: &str = "kline";
pub static AGGREGATED_TRADE: &str = "aggTrade";
pub static DEPTH_ORDERBOOK: &str = "depthUpdate";
pub static PARTIAL_ORDERBOOK: &str = "lastUpdateId";
pub static DAYTICKER: &str = "24hrTicker";
pub static MARK_PRICE: &str = "markPrice";

pub fn all_ticker_stream() -> &'static str { "!ticker@arr" }

pub fn ticker_stream(symbol: &str) -> String { format!("{symbol}@ticker") }

pub fn agg_trade_stream(symbol: &str) -> String { format!("{symbol}@aggTrade") }

pub fn trade_stream(symbol: &str) -> String { format!("{symbol}@trade") }

pub fn kline_stream(symbol: &str, interval: &str) -> String { format!("{symbol}@kline_{interval}") }

pub fn book_ticker_stream(symbol: &str) -> String { format!("{symbol}@bookTicker") }

pub fn all_book_ticker_stream() -> &'static str { "!bookTicker" }

pub fn all_mini_ticker_stream() -> &'static str { "!miniTicker@arr" }

pub fn mini_ticker_stream(symbol: &str) -> String { format!("{symbol}@miniTicker") }

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `update_speed`: 1 or 3
pub fn mark_price_stream(symbol: &str, update_speed: u8) -> String { format!("{symbol}@markPrice@{update_speed}s") }

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `levels`: 5, 10 or 20
/// * `update_speed`: 1000 or 100
pub fn partial_book_depth_stream(symbol: &str, levels: u16, update_speed: u16) -> String {
    format!("{symbol}@depth{levels}@{update_speed}ms")
}

/// # Arguments
///
/// * `symbol`: the market symbol
/// * `update_speed`: 1000 or 100
pub fn diff_book_depth_stream(symbol: &str, update_speed: u16) -> String { format!("{symbol}@depth@{update_speed}ms") }

fn combined_stream(streams: Vec<String>) -> String { streams.join("/") }

pub struct WebSockets<'a, WE> {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a + Send>,
    conf: Config,
}

impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    /// New websocket holder with default configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        Self::new_with_options(handler, Config::default())
    }

    /// New websocket holder with provided configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new_with_options<Callback>(handler: Callback, conf: Config) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a + Send,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            conf,
        }
    }

    /// Connect to multiple websocket endpoints
    /// N.B: WE has to be CombinedStreamEvent
    pub async fn connect_multiple(&mut self, endpoints: Vec<String>) -> Result<()> {
        let mut url = Url::parse(&self.conf.ws_endpoint)?;
        url.path_segments_mut()
            .map_err(|_| Error::UrlParserError(url::ParseError::RelativeUrlWithoutBase))?
            .push(STREAM_ENDPOINT);
        url.set_query(Some(&format!("streams={}", combined_stream(endpoints))));

        self.handle_connect(url.to_string()).await
    }

    /// Connect to a websocket endpoint
    pub async fn connect(&mut self, url: String) -> Result<()> {
        self.handle_connect(url).await
    }

    /// Connect to a futures websocket endpoint
    pub async fn connect_futures(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/{}/{}", self.conf.futures_ws_endpoint, WS_ENDPOINT, endpoint);

        self.handle_connect(wss).await
    }

    /// Connect to a futures websocket endpoint
    pub async fn connect_delivery(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/{}/{}", self.conf.delivery_ws_endpoint, WS_ENDPOINT, endpoint);

        self.handle_connect(wss).await
    }

    async fn handle_connect(&mut self, url: String) -> Result<()> {
        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {e}"))),
        }
    }

    /// Send a message to the WebSocket
    pub async fn send_message(&mut self, message: Message) -> Result<()> {
        if let Some((ref mut socket, _)) = self.socket {
            socket.send(message).await?;
            Ok(())
        } else {
            Err(Error::Msg("WebSocket connection is not established".to_string()))
        }
    }

    /// Disconnect from the endpoint
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub fn socket(&self) -> &Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)> { &self.socket }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        let timeout_duration = Duration::from_secs(600); // 10 minutes
        let mut last_message_time;
        while running.load(Ordering::Relaxed) {
            if let Some((ref mut socket, _)) = self.socket {
                let timeout = time::timeout(timeout_duration, socket.next()).await;
                match timeout {
                    Ok(Some(Ok(message))) => {
                        last_message_time = time::Instant::now();
                        match message {
                            Message::Text(msg) => {
                                if msg.is_empty() {
                                    return Ok(());
                                }
                                match from_str::<WE>(msg.as_str()) {
                                    Ok(event) => {
                                        if let Err(e) = (self.handler)(event) {
                                            error!("Handler error: {:?}", e);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to parse message: {:?}", e);
                                    }
                                }
                            }
                            Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
                            Message::Close(e) => {
                                return Err(Error::Msg(format!("Disconnected {e:?}")));
                            }
                        }
                    }
                    Ok(Some(Err(e))) => {
                        error!("WebSocket error: {e}");
                        return Err(Error::Msg(format!("Disconnected")));
                    }
                    Ok(None) => {
                        error!("WebSocket closed");
                        return Err(Error::Msg(format!("Disconnected")));
                    }
                    Err(_) => {
                        return Err(Error::Msg(format!("Disconnected, No data received in last 10 mins")));
                    }
                }
                if last_message_time.elapsed() >= timeout_duration {
                    error!("No data received in 10 minutes, restarting WebSocket");
                    break;
                }
            }
        }
        Ok(())
    }


}
impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    // Other methods...

    /// Check if the WebSocket is closed
    pub fn is_closed(&self) -> bool {
        self.socket.is_none()
    }
}