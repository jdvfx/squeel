#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]

use sqlx::sqlite;
use sqlx::sqlite::SqlitePool;
use sqlx::sqlite::SqliteRow;
use sqlx::Connection;
use sqlx::Row;
use std::env;

#[derive(sqlx::FromRow, Debug)]
struct Version {
    id: i64,
    source: String,
    datapath: String,
    depend: String,
    approved: u8,
    status: u8,
    asset_id: i64,
}
impl From<&SqliteRow> for Version {
    fn from(row: &SqliteRow) -> Version {
        Version {
            id: row.try_get("id").unwrap_or(0_i64),
            source: row.try_get("source").unwrap_or("_".to_string()),
            datapath: row.try_get("datapath").unwrap_or("_".to_string()),
            depend: row.try_get("depend").unwrap_or("_".to_string()),
            approved: row.try_get("approved").unwrap_or(0),
            status: row.try_get("status").unwrap_or(0),
            asset_id: row.try_get("asset_id").unwrap_or(0),
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
struct Asset {
    id: i64,
    name: String,
}
impl From<&SqliteRow> for Asset {
    fn from(row: &SqliteRow) -> Asset {
        Asset {
            id: row.try_get("id").unwrap_or(0_i64),
            name: row.try_get("name").unwrap_or("_".to_string()),
        }
    }
}

async fn create_assets_table(db_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(db_name).await?;
    let ct_assets = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "assets" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "name"	TEXT
            );
        "#,
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

async fn create_versions_table(db_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(&db_name).await?;
    let ct_assets = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "versions" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "source"	TEXT,
                "datapath"	TEXT,
                "depend"	TEXT,
                "approved"	INTEGER,
                "status"	INTEGER,
                "asset_id"	INTEGER,
                FOREIGN KEY("asset_id") REFERENCES "assets"("id")
            );
        "#,
    )
    .execute(&mut conn)
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    //
    let db_name = "sqlite:/home/bunker/assets2.db";
    //
    create_assets_table(&db_name).await?;
    create_versions_table(&db_name).await?;
    //
    Ok(())
}
