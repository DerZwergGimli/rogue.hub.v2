//! API implementation for the Star Atlas endpoints
//!
//! This module provides the staratlas-exchanges [GET], staratlas-player [GET],
//! and staratlas-tokens [GET] endpoints as defined in the guidelines.

use db::{DbPool, Player, Token};
use db::queries::staratlas;

use poem_openapi::{
    param::Query, payload::Json, ApiResponse,
    Object,
    OpenApi,
    Tags,
};

/// Tags for the Star Atlas API
#[derive(Tags)]
enum StarAtlasTags {
    /// Operations related to Star Atlas exchanges
    Exchanges,
    /// Operations related to Star Atlas players
    Players,
    /// Operations related to Star Atlas tokens
    Tokens,
}

/// API implementation for the Star Atlas endpoints
pub struct StarAtlasApi {
    /// Database connection pool
    db_pool: DbPool,
}

/// Player response object
#[derive(Debug, Object)]
struct PlayerResponse {
    /// Unique identifier for the player
    id: i32,
    /// Wallet address of the player
    wallet_address: String,
    /// Username of the player (if available)
    username: Option<String>,
    /// First seen timestamp (ISO 8601 format)
    first_seen: String,
    /// Last active timestamp (ISO 8601 format)
    last_active: String,
}

/// Token response object
#[derive(Debug, Object)]
struct TokenResponse {
    /// Unique identifier for the token
    id: i32,
    /// Mint address of the token
    mint: String,
    /// Name of the token (if available)
    name: Option<String>,
    /// Symbol of the token (if available)
    symbol: Option<String>,
    /// Type of the token (if available)
    token_type: Option<String>,
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
enum GetPlayerResponse {
    #[oai(status = 200)]
    Player(Json<PlayerResponse>),
    #[oai(status = 200)]
    Players(Json<Vec<PlayerResponse>>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    DBError,
}

#[derive(ApiResponse)]
enum GetTokenResponse {
    #[oai(status = 200)]
    Token(Json<TokenResponse>),
    #[oai(status = 200)]
    Tokens(Json<Vec<TokenResponse>>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    DBError,
}

#[derive(ApiResponse)]
enum GetExchangeResponse {
    #[oai(status = 200)]
    Exchange(Json<ExchangeResponse>),
    #[oai(status = 200)]
    Exchanges(Json<Vec<ExchangeResponse>>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    DBError,
}

impl From<Player> for PlayerResponse {
    fn from(player: Player) -> Self {
        Self {
            id: player.id,
            wallet_address: player.wallet_address,
            username: player.username,
            first_seen: player.first_seen.to_rfc3339(),
            last_active: player.last_active.to_rfc3339(),
        }
    }
}

impl From<Token> for TokenResponse {
    fn from(token: Token) -> Self {
        Self {
            id: token.id,
            mint: token.mint,
            name: token.name,
            symbol: token.symbol,
            token_type: token.token_type,
        }
    }
}

impl StarAtlasApi {
    /// Creates a new instance of the Star Atlas API
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[OpenApi]
impl StarAtlasApi {
    /// Get Star Atlas players
    ///
    /// Returns a list of Star Atlas players. Can be filtered by wallet address.
    #[oai(
        path = "/staratlas/player",
        method = "get",
        tag = "StarAtlasTags::Players"
    )]
    async fn get_staratlas_players(
        &self,
        /// Filter by wallet address
        #[oai(name = "wallet_address")]
        wallet_address: Query<Option<String>>,
    ) -> GetPlayerResponse {
        let players = if let Some(wallet_address) = wallet_address.0 {
            match staratlas::get_player_by_wallet_address(&self.db_pool, &wallet_address).await {
                Ok(Some(player)) => Some(vec![player]),
                Ok(None) => Some(vec![]),
                Err(_) => None,
            }
        } else {
            match staratlas::get_all_players(&self.db_pool).await {
                Ok(players) => Some(players),
                Err(_) => None,
            }
        };

        match players {
            None => GetPlayerResponse::DBError,
            Some(players) => {
                if players.is_empty() {
                    GetPlayerResponse::NotFound
                } else {
                    let player_responses = players.into_iter().map(PlayerResponse::from).collect();
                    GetPlayerResponse::Players(Json(player_responses))
                }
            }
        }
    }

    /// Get Star Atlas tokens
    ///
    /// Returns a list of Star Atlas tokens.
    #[oai(
        path = "/staratlas/tokens",
        method = "get",
        tag = "StarAtlasTags::Tokens"
    )]
    async fn get_staratlas_tokens(&self) -> GetTokenResponse {
        match staratlas::get_all_tokens(&self.db_pool).await {
            Ok(tokens) => {
                if tokens.is_empty() {
                    GetTokenResponse::NotFound
                } else {
                    let token_responses = tokens.into_iter().map(TokenResponse::from).collect();
                    GetTokenResponse::Tokens(Json(token_responses))
                }
            }
            Err(_) => GetTokenResponse::DBError,
        }
    }

    /// Get Star Atlas exchanges
    ///
    /// Returns a list of Star Atlas exchanges. Can be filtered by buyer, seller, or asset.
    #[oai(
        path = "/staratlas/exchanges",
        method = "get",
        tag = "StarAtlasTags::Exchanges"
    )]
    async fn get_staratlas_exchanges(
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
    ) -> GetExchangeResponse {
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
            None => GetExchangeResponse::DBError,
            Some(exchanges) => {
                if exchanges.is_empty() {
                    GetExchangeResponse::NotFound
                } else {
                    let mut exchange_responses = Vec::new();

                    for exchange in exchanges {
                        // Resolve buyer ID to wallet address
                        let buyer_wallet = match staratlas::get_player_by_id(&self.db_pool, exchange.buyer).await {
                            Ok(Some(player)) => player.wallet_address,
                            _ => exchange.buyer.to_string(), // Fallback to ID as string if player not found
                        };

                        // Resolve seller ID to wallet address
                        let seller_wallet = match staratlas::get_player_by_id(&self.db_pool, exchange.seller).await {
                            Ok(Some(player)) => player.wallet_address,
                            _ => exchange.seller.to_string(), // Fallback to ID as string if player not found
                        };

                        // Resolve asset ID to mint address
                        let asset_mint = match staratlas::get_token_by_id(&self.db_pool, exchange.asset).await {
                            Ok(Some(token)) => token.mint,
                            _ => exchange.asset.to_string(), // Fallback to ID as string if token not found
                        };

                        // Resolve pair ID to mint address
                        let pair_mint = match staratlas::get_token_by_id(&self.db_pool, exchange.pair).await {
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

                    GetExchangeResponse::Exchanges(Json(exchange_responses))
                }
            }
        }
    }
}
