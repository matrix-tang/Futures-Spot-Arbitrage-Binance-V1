use crate::binance::client::Client;
use crate::binance::config::Config;
use crate::binance::errors::*;
use crate::binance::rest_model::*;
use crate::binance::util::build_signed_request_p;
use crate::conf::C;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FuturesOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub time_in_force: Option<TimeInForce>,
    pub recv_window: Option<u64>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FuturesGetOrderRequest {
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    #[serde(rename = "origClientOrderId")]
    pub orig_client_order_id: Option<String>,
}

/// Order Request
/// perform an order for the account
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    /// A unique id for the order, automatically generated if not sent.
    pub new_client_order_id: Option<String>,
    /// Used with stop loss, stop loss limit, take profit and take profit limit order types.
    pub stop_price: Option<f64>,
    /// Used with limit, stop loss limit and take profit limit to create an iceberg order.
    pub iceberg_qty: Option<f64>,
    /// Set the response json, market and limit default to full others to ack.
    pub new_order_resp_type: Option<OrderResponse>,
    /// Cannot be greater than 60000
    pub recv_window: Option<u64>,
}

impl OrderRequest {
    fn valid(&self) -> Result<()> {
        if self.iceberg_qty.is_some() && self.time_in_force != Some(TimeInForce::GTC) {
            return Err(Error::InvalidOrderError {
                msg: "Time in force has to be GTC for iceberg orders".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusRequest {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
    /// Cannot be greater than 60000
    pub recv_window: Option<u64>,
}

#[derive(Clone)]
pub struct MyApi {
    pub client: Client,
    pub futures_client: Client,
    pub delivery_client: Client,
    pub recv_window: u64,
}

impl MyApi {
    pub fn new() -> Self {
        MyApi {
            client: Client::new(
                Some(C.binance_api_config.api_key.clone()),
                Some(C.binance_api_config.secret_key.clone()),
                Config::default().rest_api_endpoint,
                Some(5),
            ),
            futures_client: Client::new(
                Some(C.binance_api_config.api_key.clone()),
                Some(C.binance_api_config.secret_key.clone()),
                Config::default().futures_rest_api_endpoint,
                Some(5),
            ),
            delivery_client: Client::new(
                Some(C.binance_api_config.api_key.clone()),
                Some(C.binance_api_config.secret_key.clone()),
                Config::default().delivery_rest_api_endpoint,
                Some(5),
            ),
            recv_window: 5000,
        }
    }

    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get("/api/v3/time", None).await
    }

    pub async fn universal_transfer(
        &self,
        asset: String,
        amount: f64,
        transfer_type: UniversalTransferType,
    ) -> Result<TransactionId> {
        let transfer = UniversalTransfer {
            asset,
            amount,
            from_symbol: None,
            to_symbol: None,
            transfer_type: transfer_type,
        };

        self.client
            .post_signed_p("/sapi/v1/asset/transfer", transfer, self.recv_window)
            .await
    }

    pub async fn place_order(&self, order: OrderRequest) -> Result<Transaction> {
        order.valid()?;
        let recv_window = order.recv_window.unwrap_or(self.recv_window);
        let request = build_signed_request_p(order, recv_window)?;
        self.client.post_signed("/api/v3/order", &request).await
    }

    pub async fn order_status(&self, osr: OrderStatusRequest) -> Result<Order> {
        let recv_window = osr.recv_window.unwrap_or(self.recv_window);
        let request = build_signed_request_p(osr, recv_window)?;
        self.client.get_signed("/api/v3/order", &request).await
    }

    /// Get an order
    pub async fn futures_order_status(
        &self,
        order: FuturesGetOrderRequest,
    ) -> Result<FuturesTransaction> {
        self.futures_client
            .get_signed_p("/fapi/v1/order", Some(order), self.recv_window)
            .await
    }

    /// Place an order
    pub async fn futures_place_order(
        &self,
        order: FuturesOrderRequest,
    ) -> Result<FuturesTransaction> {
        self.futures_client
            .post_signed_p("/fapi/v1/order", order, self.recv_window)
            .await
    }

    /// Get an order
    pub async fn delivery_order_status(
        &self,
        order: FuturesGetOrderRequest,
    ) -> Result<FuturesTransaction> {
        self.delivery_client
            .get_signed_p("/dapi/v1/order", Some(order), self.recv_window)
            .await
    }

    /// Place an order
    pub async fn delivery_place_order(
        &self,
        order: FuturesOrderRequest,
    ) -> Result<FuturesTransaction> {
        self.delivery_client
            .post_signed_p("/dapi/v1/order", order, self.recv_window)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_binance_api() {
        let api = MyApi::new();
        // let time = api.get_server_time().await;
        // println!("{:?}", time);
        /*let transfer = api
            .universal_transfer("USDT".to_string(), 1.0, UniversalTransferType::MainUmfuture)
            .await;
        println!("{:?}", transfer);*/

        /*let x = api
            .place_order(OrderRequest {
                symbol: "BNBUSDT".to_string(),
                quantity: Some(0.02),
                price: Some(500.0),
                order_type: OrderType::Limit,
                side: OrderSide::Buy,
                time_in_force: Some(TimeInForce::FOK),
                ..OrderRequest::default()
            })
            .await;
        println!("{:?}", x);*/

        let s = api
            .order_status(OrderStatusRequest {
                symbol: "BNBUSDT".to_string(),
                order_id: Some(5096738273),
                orig_client_order_id: None,
                recv_window: None,
            })
            .await;
        println!("{:?}", s);
    }

    #[tokio::test]
    async fn test_binance_futures_place_order() {
        let api = MyApi::new();
        let transaction = api
            .futures_place_order(FuturesOrderRequest {
                symbol: "ETHUSDT".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                quantity: Some(0.1),
                price: Some(3950.0),
                time_in_force: Some(TimeInForce::GTC),
                recv_window: None,
            })
            .await;
        println!("{:?}", transaction);
    }

    #[tokio::test]
    async fn test_binance_futures_order_status() {
        let api = MyApi::new();
        let transaction = api
            .futures_order_status(FuturesGetOrderRequest {
                symbol: "ETHUSDT".to_string(),
                order_id: Some("8389765661061305976".to_string()),
                orig_client_order_id: None,
            })
            .await;
        println!("{:?}", transaction);
    }

    #[tokio::test]
    async fn test_binance_delivery_place_order() {
        let api = MyApi::new();
        let transaction = api
            .delivery_place_order(FuturesOrderRequest {
                symbol: "ETHUSD_PERP".to_string(),
                side: OrderSide::Buy,
                order_type: OrderType::Limit,
                quantity: Some(1.0),
                price: Some(140.0),
                time_in_force: Some(TimeInForce::GTC),
                recv_window: None,
            })
            .await;
        println!("{:?}", transaction);
    }

    #[tokio::test]
    async fn test_binance_delivery_order_status() {
        let api = MyApi::new();
        let transaction = api
            .delivery_order_status(FuturesGetOrderRequest {
                symbol: "ETHUSD_PERP".to_string(),
                order_id: Some("73992692757".to_string()),
                orig_client_order_id: None,
            })
            .await;
        println!("{:?}", transaction);
    }
}
