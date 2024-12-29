use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PriceResponse {
    bitcoin: CryptoPrice,
    ethereum: CryptoPrice,
}

#[derive(Deserialize, Debug)]
struct CryptoPrice {
    usd: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL do API CoinGecko
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

    // Tworzenie klienta HTTP
    let client = Client::new();

    // Wysyłanie żądania GET do API i deserializacja odpowiedzi JSON
    let response: PriceResponse = client.get(url).send()?.json()?;

    // Wyświetlanie cen kryptowalut
    println!("Bitcoin (BTC) price in USD: ${}", response.bitcoin.usd);
    println!("Ethereum (ETH) price in USD: ${}", response.ethereum.usd);

    Ok(())
}