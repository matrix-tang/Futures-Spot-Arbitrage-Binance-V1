#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let request = None;
    let url = request
        .map(|r: &str| {
            format!(
                "{}{}?{}",
                "https://data-api.binance.vision", "/api/v3/exchangeInfo", r
            )
        })
        .unwrap_or_else(|| {
            format!(
                "{}{}",
                "https://data-api.binance.vision", "/api/v3/exchangeInfo"
            )
        });

    let response = client.get(&url).send().await?;

    let x = response.status();

    let j = response.text().await?;
    println!("{:?} {:?} {:?}", url, x, j);

    /*let body = reqwest::get("https://api.binance.com/api/v3/exchangeInfo")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);*/

    Ok(())
}
