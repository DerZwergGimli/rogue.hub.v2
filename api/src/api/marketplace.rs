//! API implementation for the marketplace endpoint
//!
//! This module provides the marketplace-exchanges [GET] endpoint, which returns
//! marketplace exchange data.

use db::{DbPool, Exchange};
use db::queries::staratlas;

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
    /// Buyer wallet address
    buyer: String,
    /// Seller wallet address
    seller: String,
    /// Asset mint address
    asset: String,
    /// Pair mint address
    pair: String,
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

// We can't implement From<Exchange> for ExchangeResponse anymore because we need to resolve
// foreign keys to addresses, which requires async functions. Instead, we'll create
// ExchangeResponse instances directly in the get_marketplace_exchanges function.

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
                let mut exchange_responses = Vec::new();

                for exchange in exchanges {
                    // Resolve buyer ID to wallet address
                    let buyer_wallet = match db::queries::staratlas::get_player_by_id(&self.db_pool, exchange.buyer).await {
                        Ok(Some(player)) => player.wallet_address,
                        _ => exchange.buyer.to_string(), // Fallback to ID as string if player not found
                    };

                    // Resolve seller ID to wallet address
                    let seller_wallet = match db::queries::staratlas::get_player_by_id(&self.db_pool, exchange.seller).await {
                        Ok(Some(player)) => player.wallet_address,
                        _ => exchange.seller.to_string(), // Fallback to ID as string if player not found
                    };

                    // Resolve asset ID to mint address
                    let asset_mint = match db::queries::staratlas::get_token_by_id(&self.db_pool, exchange.asset).await {
                        Ok(Some(token)) => token.mint,
                        _ => exchange.asset.to_string(), // Fallback to ID as string if token not found
                    };

                    // Resolve pair ID to mint address
                    let pair_mint = match db::queries::staratlas::get_token_by_id(&self.db_pool, exchange.pair).await {
                        Ok(Some(token)) => token.mint,
                        _ => exchange.pair.to_string(), // Fallback to ID as string if token not found
                    };

                    exchange_responses.push(ExchangeResponse {
                        id: exchange.id,
                        slot: exchange.slot,
                        signature: exchange.signature.to_string(),
                        index: exchange.index,
                        timestamp: exchange.timestamp.to_rfc3339(),
                        side: exchange.side.to_string(),
                        buyer: buyer_wallet,
                        seller: seller_wallet,
                        asset: asset_mint,
                        pair: pair_mint,
                        price: exchange.price,
                        size: exchange.size,
                        volume: exchange.volume,
                        fee: exchange.fee,
                        buddy: exchange.buddy,
                    });
                }

                GetMarketplaceResponse::Exchnages(Json(exchange_responses))
            }
        }
    }
}
