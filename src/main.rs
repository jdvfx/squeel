#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]
use sqlx::sqlite::SqliteRow;

use sqlx::sqlite;
use sqlx::Connection;
use sqlx::Row;

#[derive(sqlx::FromRow, Debug)]
struct Dbstruct {
    id: i64,
    first_name: String,
    last_name: String,
    age: i8,
    gender: String,
}

// need to use try_get() instead of get()
// otherwise, it's going to panic!
// if any field is missing
impl From<&SqliteRow> for Dbstruct {
    fn from(row: &SqliteRow) -> Dbstruct {
        Dbstruct {
            id: row.try_get("id").unwrap_or(0),
            first_name: row.try_get("first_name").unwrap_or("_".to_string()),
            last_name: row.try_get("last_name").unwrap_or("_".to_string()),
            age: row.try_get("age").unwrap_or(0),
            gender: row.try_get("gender").unwrap_or("_".to_string()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect("sqlite:/home/bunker/test1.db").await?;

    let row = sqlx::query("SELECT * FROM people")
        .fetch_all(&mut conn)
        .await?;

    for i in row.iter() {
        let x: Dbstruct = i.into();
        println!("{:?}", x);
    }

    Ok(())
}

// > shows
// show_id, show_name

// > sequences
// seq_id, seq_name , show_id*

// > shots
// shot_id, shot_name , seq_id*

// > assets
// asset_id (u32 = INTEGER)
// asset_name (String = TEXT)
// asset_location (Vec<String> = TEXT)

// > versions
// - asset_id (u32 = INTEGER) *
// - version (u32 = INTEGER)
// - datapath = "/path" (String = TEXT)
// - source = "source" (String = TEXT)
// - approved = bool
// - status = "Online"
