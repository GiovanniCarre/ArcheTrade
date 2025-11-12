use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericStockDataDTO {
    /// Symbole de l'action
    pub symbol: String,

    /// Fournisseur de la donnée
    pub provider: Option<String>,

    /// Date ou timestamp de l'enregistrement
    pub date: NaiveDate,

    /// Prix d'ouverture
    pub open: Option<f64>,

    /// Prix de clôture
    pub close: Option<f64>,

    /// Plus haut du jour
    pub high: Option<f64>,

    /// Plus bas du jour
    pub low: Option<f64>,

    /// Volume échangé
    pub volume: Option<f64>,

    /// Champs supplémentaires dynamiques, pour futur usage
    pub extra: Option<serde_json::Value>,
}

impl GenericStockDataDTO {
    pub fn new(symbol: &str, date: NaiveDate) -> Self {
        Self {
            symbol: symbol.to_string(),
            provider: None,
            date,
            open: None,
            close: None,
            high: None,
            low: None,
            volume: None,
            extra: None,
        }
    }
}
