# Futures-Spot-Arbitrage-Binance-V1

This project aims to implement an arbitrage strategy between Binance Futures and Spot markets. By exploiting price
differences between these two markets, the strategy seeks to generate profits.

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
- Rust libraries: `reqwest`, `tokio`, `binance-rs-async` ..

## Setup

1. Clone this repository:

   ```
   git clone https://github.com/your-username/binance-futures-spot-arbitrage.git
   ```

2. Install required Python packages:

   ```
   pip install -r requirements.txt
   ```

3. Obtain API keys from Binance with Futures and Spot trading permissions.

4. Set up API keys in `config.py`:

   ```python
   BINANCE_API_KEY = 'your_binance_api_key'
   BINANCE_API_SECRET = 'your_binance_api_secret'
   ```

## Usage

Run the main script to start monitoring and executing arbitrage opportunities:

```
python main.py
```

## Configuration

Adjust configuration parameters in `config.py` according to your preferences and risk tolerance.

## Disclaimer

- **Use at Your Own Risk**: Trading involves risks, and past performance is not indicative of future results. Always
  exercise caution and perform thorough research before engaging in trading activities.
- **Not Financial Advice**: This project is for educational and informational purposes only. It does not constitute
  financial advice, and the authors are not responsible for any losses incurred from using this software.

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests to enhance the project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

**Disclaimer**: This software is provided "as is", without warranty of any kind, express or implied, including but not
limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall
the authors or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract,
tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the
software.