use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::get,
    Json, Router,
};
use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio_postgres::{NoTls, Row};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use postgres_from_row::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
struct Default {
    id: String,
    timestamp: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let manager: PostgresConnectionManager<NoTls> = PostgresConnectionManager::new_from_stringlike(
        "user=postgres password=password dbname=kitty host=192.168.1.3",
        NoTls,
    )
    .unwrap();

    // set up connection pool
    let pool: Pool<PostgresConnectionManager<NoTls>> =
        Pool::builder().build(manager).await.unwrap();

    // build our application with some routes
    let app = Router::new()
        .route(
            "/",
            get(using_connection_pool_extractor), // .post(using_connection_extractor),
        )
        .with_state(pool);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

async fn using_connection_pool_extractor(
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let values: Vec<Row> = conn
        .query(
            "SELECT id::TEXT, timestamp::TEXT
                    FROM S_KITTY.KITTY_DATA
                    GROUP BY ID
                    LIMIT 100",
            &[],
        )
        .await
        .map_err(internal_error)?;

    let parsed:Vec<Default>  = values.iter().map(|x| Default::try_from_row(&x).unwrap()).collect();

    let deserialized: String = serde_json::to_string(&parsed).unwrap();

    Ok(deserialized)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(PooledConnection<'static, PostgresConnectionManager<NoTls>>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    ConnectionPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = ConnectionPool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

// async fn using_connection_extractor(
//     DatabaseConnection(conn): DatabaseConnection,
// ) -> Result<String, (StatusCode, String)> {
//     let row = conn
//         .query_one("select 1 + 1", &[])
//         .await
//         .map_err(internal_error)?;
//     let two: i32 = row.try_get(0).map_err(internal_error)?;

//     Ok(two.to_string())
// }

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
