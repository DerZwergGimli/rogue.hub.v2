//! API implementation for the indexer endpoint
//!
//! This module provides the indexer [GET] endpoint, which serves a simple HTML table
//! to view the indexers.

use db::{DbPool, Indexer as IndexerDB, PublicKeyType, SignatureType};
use poem_openapi::{
    payload::{Html, Json}, ApiResponse, Object, OpenApi,
    Tags,
};

/// Tags for the indexer API
#[derive(Tags)]
enum IndexerTags {
    /// Operations related to indexers
    Indexers,
}

/// API implementation for the indexer endpoint
pub struct IndexerApi {
    /// Database connection pool
    db_pool: DbPool,
}

impl IndexerApi {
    /// Creates a new instance of the indexer API
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    /// Generates an HTML table for the indexers
    fn generate_html_table(indexers: &[IndexerDB]) -> String {
        let mut html = String::from(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Indexers</title>
                <style>
                    table {
                        border-collapse: collapse;
                        width: 100%;
                    }
                    th, td {
                        border: 1px solid #ddd;
                        padding: 8px;
                        text-align: left;
                    }
                    th {
                        background-color: #f2f2f2;
                    }
                    tr:nth-child(even) {
                        background-color: #f9f9f9;
                    }
                </style>
            </head>
            <body>
                <h1>Indexers</h1>
                <table>
                    <tr>
                        <th>ID</th>
                        <th>Name</th>
                        <th>Direction</th>
                        <th>Program ID</th>
                        <th>Before Signature</th>
                        <th>Until Signature</th>
                        <th>Before Block</th>
                        <th>Until Block</th>
                        <th>Finished</th>
                        <th>Fetch Limit</th>
                    </tr>
            "#,
        );

        for indexer in indexers {
            html.push_str(&format!(
                r#"
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{:?}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td>{}</td>
                </tr>
                "#,
                indexer.id,
                indexer.name.as_deref().unwrap_or(""),
                indexer.direction,
                indexer.program_id,
                indexer
                    .before_signature
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                indexer
                    .until_signature
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                indexer
                    .before_block
                    .map(|b| b.to_string())
                    .unwrap_or_default(),
                indexer
                    .until_block
                    .map(|b| b.to_string())
                    .unwrap_or_default(),
                indexer.finished.map(|f| f.to_string()).unwrap_or_default(),
                indexer.fetch_limit,
            ));
        }

        html.push_str(
            r#"
                </table>
            </body>
            </html>
            "#,
        );

        html
    }
}

#[derive(Object)]
struct Indexer {
    id: i32,
    name: Option<String>,
    direction: String,
    program_id: PublicKeyType,
    before_signature: Option<SignatureType>,
    until_signature: Option<SignatureType>,
    before_block: Option<i64>,
    until_block: Option<i64>,
    finished: Option<bool>,
    fetch_limit: i32,
}

#[derive(ApiResponse)]
enum GetIndexerResponse {
    #[oai(status = 200)]
    Indexer(Json<Indexer>),
    #[oai(status = 200)]
    Indexers(Json<Vec<Indexer>>),
    #[oai(status = 200)]
    HTML(Html<String>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    DBError,
}

#[OpenApi]
impl IndexerApi {
    /// Get all indexers as JSON
    ///
    /// Returns a list of all indexers in the database as JSON.
    #[oai(path = "/indexers", method = "get", tag = "IndexerTags::Indexers")]
    async fn get_indexers_json(&self) -> GetIndexerResponse {
        match db::get_all_indexers(&self.db_pool).await {
            Ok(indexers) => {
                let response = indexers
                    .into_iter()
                    .map(|indexer| Indexer {
                        id: indexer.id,
                        name: indexer.name,
                        direction: indexer.direction.to_string(),
                        program_id: indexer.program_id,
                        before_signature: indexer.before_signature,
                        until_signature: indexer.until_signature,
                        before_block: indexer.before_block,
                        until_block: indexer.until_block,
                        finished: indexer.finished,
                        fetch_limit: indexer.fetch_limit,
                    })
                    .collect();

                GetIndexerResponse::Indexers(Json(response))
            }
            Err(_) => GetIndexerResponse::DBError,
        }
    }

    /// Get all indexers as HTML
    ///
    /// Returns a simple HTML table to view the indexers.
    #[oai(path = "/indexer", method = "get", tag = "IndexerTags::Indexers")]
    async fn get_indexers_html(&self) -> GetIndexerResponse {
        match db::get_all_indexers(&self.db_pool).await {
            Ok(indexers) => {
                let html = Self::generate_html_table(&indexers);
                GetIndexerResponse::HTML(Html(html))
            }

            Err(_) => GetIndexerResponse::DBError,
        }
    }
}
