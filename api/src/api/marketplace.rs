//! API implementation for the marketplace endpoint
//!
//! This module provides the marketplace-exchanges [GET] endpoint, which returns
//! marketplace exchange data.

use db::{DbPool, Exchange};

use poem_openapi::payload::Html;
use poem_openapi::{
    param::Query, payload::Json, types::{ToJSON, Type}, ApiResponse,
    Object,
    OpenApi,
    Tags,
};

/// Tags for the marketplace API
#[derive(Tags)]
enum MarketplaceTags {
    /// Operations related to marketplace exchanges
    Exchanges,
}

/// API implementation for the marketplace endpoint
pub struct MarketplaceApi {
    /// Database connection pool
    db_pool: DbPool,
}

/// Exchange response object
#[derive(Debug, Object)]
struct ExchangeResponse {
    /// Unique identifier for the exchange
    id: i32,
    /// Block number of the exchange
    slot: i32,
    /// Transaction signature
    signature: String,
    /// Index within the transaction
    index: i32,
    /// Timestamp of the exchange (ISO 8601 format)
    timestamp: String,
    /// Side of the exchange (buy/sell)
    side: String,
    /// Buyer ID
    buyer: i32,
    /// Seller ID
    seller: i32,
    /// Asset ID
    asset: i32,
    /// Pair ID
    pair: i32,
    /// Price of the exchange
    price: f64,
    /// Size of the exchange
    size: i32,
    /// Volume of the exchange
    volume: f64,
    /// Fee of the exchange
    fee: f64,
    /// Buddy fee of the exchange
    buddy: f64,
}

#[derive(ApiResponse)]
enum GetMarketplaceResponse {
    #[oai(status = 200)]
    Exchnage(Json<ExchangeResponse>),
    #[oai(status = 200)]
    Exchnages(Json<Vec<ExchangeResponse>>),
    #[oai(status = 200)]
    HTML(Html<String>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    DBError,
}

impl From<Exchange> for ExchangeResponse {
    fn from(exchange: Exchange) -> Self {
        Self {
            id: exchange.id,
            slot: exchange.slot,
            signature: exchange.signature,
            index: exchange.index,
            timestamp: exchange.timestamp.to_rfc3339(),
            side: exchange.side,
            buyer: exchange.buyer,
            seller: exchange.seller,
            asset: exchange.asset,
            pair: exchange.pair,
            price: exchange.price,
            size: exchange.size,
            volume: exchange.volume,
            fee: exchange.fee,
            buddy: exchange.buddy,
        }
    }
}

impl MarketplaceApi {
    /// Creates a new instance of the marketplace API
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[OpenApi]
impl MarketplaceApi {
    /// Get marketplace exchanges
    ///
    /// Returns a list of marketplace exchanges. Can be filtered by buyer, seller, or asset.
    #[oai(
        path = "/marketplace-exchanges",
        method = "get",
        tag = "MarketplaceTags::Exchanges"
    )]
    async fn get_marketplace_exchanges(
        &self,
        /// Filter by buyer ID
        #[oai(name = "buyer_id")]
        buyer_id: Query<Option<i32>>,
        /// Filter by seller ID
        #[oai(name = "seller_id")]
        seller_id: Query<Option<i32>>,
        /// Filter by asset ID
        #[oai(name = "asset_id")]
        asset_id: Query<Option<i32>>,
    ) -> GetMarketplaceResponse {
        let exchanges = if let Some(buyer_id) = buyer_id.0 {
            match db::get_exchanges_by_buyer_id(&self.db_pool, buyer_id).await {
                Ok(exchanges) => Some(exchanges),
                Err(_) => None,
            }
        } else if let Some(seller_id) = seller_id.0 {
            match db::get_exchanges_by_seller_id(&self.db_pool, seller_id).await {
                Ok(exchanges) => Some(exchanges),
                Err(_) => None,
            }
        } else if let Some(asset_id) = asset_id.0 {
            match db::get_exchanges_by_asset_id(&self.db_pool, asset_id).await {
                Ok(exchanges) => Some(exchanges),
                Err(_) => None,
            }
        } else {
            match db::get_all_exchanges(&self.db_pool).await {
                Ok(exchanges) => Some(exchanges),
                Err(_) => None,
            }
        };

        match exchanges {
            None => GetMarketplaceResponse::DBError,
            Some(exchanges) => {
                let exchange_responses = exchanges
                    .into_iter()
                    .map(|exchange| ExchangeResponse {
                        id: exchange.id,
                        slot: exchange.slot,
                        signature: exchange.signature.to_string(),
                        index: exchange.index,
                        timestamp: exchange.timestamp.to_rfc3339(),
                        side: exchange.side.to_string(),
                        buyer: exchange.buyer,
                        seller: exchange.seller,
                        asset: exchange.asset,
                        pair: exchange.pair,
                        price: exchange.price,
                        size: exchange.size,
                        volume: exchange.volume,
                        fee: exchange.fee,
                        buddy: exchange.buddy,
                    })
                    .collect();
                GetMarketplaceResponse::Exchnages(Json(exchange_responses))
            }
        }
    }
}
