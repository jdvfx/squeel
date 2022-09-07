use sqlx::sqlite;
use sqlx::Connection;
mod assetdef;
//
async fn create_assets_table(db_name: &str) -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect(db_name).await?;
    // TO DO : use the result of _ct_assets_
    let _ct_assets = sqlx::query(
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
    // TO DO : use the result of _ct_assets_
    let _ct_assets = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS "versions" (
                "id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
                "version"	INTEGER,
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
