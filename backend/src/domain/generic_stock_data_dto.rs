use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::domain::stock_insights::StockInsights;
use crate::domain::stock_insights_builder::StockInsightsBuilder;
use crate::domain::time_series::StockSegment;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericStockDataDTO {
    pub symbol: String,
    pub provider: Option<String>,
    pub last_update: Option<DateTime<Utc>>,
    pub historical_segments: Vec<StockSegment>,

    #[serde(skip)]
    insights: StockInsights,
}

impl GenericStockDataDTO {
    pub fn new(
        symbol: String,
        provider: Option<String>,
        last_update: Option<DateTime<Utc>>,
        historical_segments: Vec<StockSegment>,
    ) -> Self {
        let insights = StockInsightsBuilder::build(&historical_segments);

        Self {
            symbol,
            provider,
            last_update,
            historical_segments,
            insights,
        }
    }

    pub fn insights(&self) -> &StockInsights {
        &self.insights
    }
}
