use tokio_postgres::{Error, NoTls, Row};

pub struct Database {
    client: tokio_postgres::Client,
}

impl Database {
    pub async fn connect() -> Result<Database, Error> {
        let db_url = "postgres://postgres.vplwkeuexorgkrrcrppp:.zp?KkH.f~MV8G,@aws-0-us-west-1.pooler.supabase.com:5432/postgres";
        let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(Database { client: client })
    }

    pub async fn perform_query(&self) -> Result<Vec<Row>, Error> {
        let rows = self
            .client
            .query("SELECT * FROM public.pals_basic", &[])
            .await?;

        Ok(rows)
    }
}
