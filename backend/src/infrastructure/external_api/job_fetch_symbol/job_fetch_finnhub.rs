use crate::infrastructure::db::mongo_stock_manager::MongoStockManager;
use crate::domain::stock_summary::StockSummary;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

/// Représente un marché (exchange)
#[derive(Debug, Deserialize)]
struct Exchange {
    code: String,
    name: String,
    mic: Option<String>,
    timezone: Option<String>,
}

/// Structure renvoyée par l'API Finnhub pour les symboles
#[derive(Debug, Deserialize)]
struct FinnhubStock {
    symbol: String,
    description: String,
    mic: Option<String>,
    #[serde(rename = "displaySymbol")]
    display_symbol: Option<String>,
    #[serde(rename = "type")]
    stock_type: Option<String>,
}

/// Télécharge la liste des marchés depuis le Google Sheet exporté en CSV
async fn fetch_exchanges() -> Result<Vec<Exchange>> {
    let sheet_csv_url = "https://docs.google.com/spreadsheets/d/1I3pBxjfXB056-g_JYf_6o3Rns3BV2kMGG1nCatb91ls/export?format=csv";

    let client = Client::new();
    let response = client.get(sheet_csv_url).send().await?;
    let content = response.text().await?;

    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut exchanges = Vec::new();

    for result in rdr.deserialize() {
        let record: Exchange = result?;
        exchanges.push(record);
    }

    Ok(exchanges)
}

/// Télécharge les symboles d’un marché donné et insère dans MongoDB
async fn fetch_symbols_for_exchange(
    client: &Client,
    manager: &MongoStockManager,
    api_key: &str,
    exchange_code: &str,
) -> Result<u32> {
    let url = format!(
        "https://finnhub.io/api/v1/stock/symbol?exchange={}&token={}",
        exchange_code, api_key
    );

    println!("Url : {}", url);
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(anyhow!("Erreur API Finnhub pour exchange {}: {}", exchange_code, response.status()));
    }

    let stocks: Vec<FinnhubStock> = response.json().await?;
    let mut count = 0;

    for s in stocks {
        let symbol = s.symbol.trim();
        let name = s.description.trim();
        if symbol.is_empty() || name.is_empty() {
            continue;
        }


        let stock = StockSummary::new(symbol, name, exchange_code);
        manager.add_stock(stock).await?;
        count += 1;
    }

    println!("{} : {} actions ajoutées", exchange_code, count);
    Ok(count)
}

//télécharge les actions depuis Finnhub pour tous les marchés
pub async fn fetch_all_stocks_from_finnhub(manager: &MongoStockManager) -> Result<()> {
    dotenv().ok();
    let api_key = env::var("FINNHUB_API_KEY")
        .map_err(|_| anyhow!("FINNHUB_API_KEY doit être défini dans .env"))?;

    let exchanges = fetch_exchanges().await?;
    let client = Client::new();

    println!("{} marchés trouvés, récupération des symboles!", exchanges.len());

    let mut total = 0u32;

    for ex in exchanges {
        if ex.code.trim().is_empty() {
            continue;
        }

        match fetch_symbols_for_exchange(&client, manager, &api_key, &ex.code).await {
            Ok(n) => total += n,
            Err(e) => eprintln!("Erreur sur {} : {:?}", ex.code, e),
        }
    }

    println!("Import terminé ! Total : {} actions ajoutées à MongoDB", total);
    Ok(())
}
