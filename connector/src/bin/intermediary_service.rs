use std::collections::HashMap;
use std::sync::Arc;
use tokio_postgres::NoTls;
use warp::Filter;

#[tokio::main]
async fn main() {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=mysecretpassword dbname=test_db", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let client = Arc::new(client);

    let route = warp::post()
        .and(warp::path("execute"))
        .and(warp::body::bytes())
        .and(with_db(client.clone()))
        .and_then(handle_request);

    warp::serve(route).run(([127, 0, 0, 1], 8080)).await;
}

fn with_db(client: Arc<tokio_postgres::Client>) -> impl Filter<Extract = (Arc<tokio_postgres::Client>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}

async fn handle_request(body: bytes::Bytes, client: Arc<tokio_postgres::Client>) -> Result<impl warp::Reply, warp::Rejection> {
    let sql_query = String::from_utf8(body.to_vec()).unwrap();

    match client.execute(&sql_query, &[]).await {
        Ok(rows_affected) => Ok(warp::reply::json(&HashMap::from([("rows_affected", rows_affected)]))),
        Err(e) => {
            eprintln!("SQL execution error: {:?}", e);
            Ok(warp::reply::json(&HashMap::from([("error", e.to_string())])))
        }
    }
}
