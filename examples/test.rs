use std::sync::atomic::AtomicBool;
use arbitrage::binance::websockets::*;
use arbitrage::binance::ws_model::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let keep_running = AtomicBool::new(true);
    let all_ticker = all_mini_ticker_stream();

    let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> = WebSockets::new(|events: Vec<WebsocketEvent>| {
        for tick_events in events {
            // println!("{:?}", tick_events);
            if let WebsocketEvent::DayMiniTicker(tick_event) = tick_events {
                println!("{:?}", tick_event);
            }
        }

        Ok(())
    });

    web_socket.connect_delivery(all_ticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
    Ok(())
}