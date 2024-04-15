# Futures-Spot-Arbitrage-Binance-V1

The project includes arbitrage strategies between Binance futures and spot markets and stablecoin hedging arbitrage
strategies.

该项目包括币安期货和现货市场之间的套利策略以及稳定币对冲套利
策略。

## Features

- **Real-time Price Monitoring**: Utilizes Binance API to fetch real-time prices from both Futures and Spot markets.
- **Arbitrage Opportunity Detection**: Monitors price differentials between Futures and Spot markets to identify
  potential arbitrage opportunities.
- **Automated Trading**: Execute trades automatically when profitable arbitrage opportunities are detected.
- **Risk Management**: Includes risk management measures to mitigate potential losses.
- **Logging and Reporting**: Logs trade history and performance metrics for analysis and reporting.

## Requirements

- Rust 1.78.0-nightly
- Binance API keys with Futures and Spot trading permissions
- Rust libraries: `reqwest`, `tokio`, `binance-rs-async`, `rocksdb`, `bincode` ..

## Setup

1. Clone this repository:

   ```
   git clone https://github.com/matrix-tang/Futures-Spot-Arbitrage-Binance-V1.git
   ```

2. Obtain API keys from Binance with Futures and Spot trading permissions.

3. Set up API keys in `config.tmol`:

   ```toml
    [redis]
    url = "redis://127.0.0.1:6379/"

    [mysql]
    url = "mysql://root:123456@localhost:3306/arbitrage"
   
    [rocksdb]
    path = "_path_for_rocksdb_storage"

    [binance_api_config]
    api_key = ""
    secret_key = ""

    [log]
    pattern = "console" # console/file 控制台/文件
    dir = "logs"
    prefix = "arb.log"
    level = "INFO"
   ```

## Usage

Run the main script to start monitoring and executing arbitrage opportunities:

```shell
  // run arbitrage 
  // 执行期现套利策略
  cargo run --bin arbitrage
  // run stable coin hedging
  // 执行稳定币对冲策略
  cargo run --bin hedging
```

## Configuration

Adjust configuration parameters in `config.toml` according to your preferences and risk tolerance.

## Disclaimer

- **Use at Your Own Risk**: Trading involves risks, and past performance is not indicative of future results. Always
  exercise caution and perform thorough research before engaging in trading activities.
- **Not Financial Advice**: This project is for educational and informational purposes only. It does not constitute
  financial advice, and the authors are not responsible for any losses incurred from using this software.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests to enhance the project.

---

**Disclaimer**: This software is provided "as is", without warranty of any kind, express or implied, including but not
limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall
the authors or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract,
tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the
software.