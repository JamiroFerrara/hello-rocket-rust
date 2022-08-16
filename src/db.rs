use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{FromRow, Row, Pool, MySql};

pub struct PScale {
    pub pool: Pool<MySql>,
}

impl PScale {
    pub async fn get_all_tracks(& self) -> Result<Vec<String>, sqlx::Error>{
        let rows = sqlx::query("SELECT * FROM Track")
            .fetch_all(&self.pool)
            .await?;

        let str_result = rows
            .iter()
            .map(|r| format!("{}", r.get::<String, _>("title")))
            .collect::<Vec<String>>();

        Ok(str_result)
    }
    
}

pub async fn new() -> Result<Pool<MySql>, sqlx::Error>{
    let pscale_url = "";

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&pscale_url)
        .await?;

    Ok(pool)
}

