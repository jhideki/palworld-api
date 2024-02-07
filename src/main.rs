use tokio;
mod db;
use db::Database;

#[tokio::main]
async fn main() {
    let database = Database::connect().await.unwrap();
    match database.perform_query().await {
        Ok(rows) => {
            let value: i64 = rows[0].get(0);
            println!("{}", value);
        }
        Err(e) => eprintln!("Error quering data: {}", e),
    }
}
