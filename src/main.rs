
#![allow(dead_code, unused_variables, unused_assignments, unused_imports)]
use sqlx::sqlite::SqliteRow;

// use serde::{Deserialize, Serialize};

use sqlx::Row;
use sqlx::sqlite;
use sqlx::Connection;

#[derive(sqlx::FromRow, Debug)]
struct Dbstruct {
    id: i64,
    first_name: String,
    last_name: String,
    age: i8,
    gender: String,
}

impl From<&SqliteRow> for Dbstruct {
    fn from(row: &SqliteRow) -> Dbstruct {
        Dbstruct {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            age: row.get("age"),
            gender: row.get("gender"),
        }
    }
}

// fn row_deser(row: Vec<SqliteRow>) -> Dbstruct {
//     let id: Result<i64, _> = row.try_get("id");
//     println!("id: {}", id);
// }

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let mut conn = sqlite::SqliteConnection::connect("sqlite:/home/bunker/test1.db").await?;

    let row = sqlx::query("SELECT * FROM people")
        .fetch_all(&mut conn)
        .await?;

    for i in row.iter() {
        let id: Result<i64, sqlx::Error> = i.try_get("id");
        match id {
            Ok(id) => println!("Ok: {}", id),
            Err(e) => println!("Err: {}", e),
        }

        let x: Dbstruct = i.into();
        println!("{:?}", x);

        // let id: Option<i64> = i.get("idx");
        // println!("{:?}",id);
        // let first_name: String = i.get("first_name");
        // let last_name: String = i.get("last_name");
        // let age: i64 = i.get("age");
        // let gender: String = i.get("gender");
        // let x: String = i.get("x");

        // println!("{id} {first_name} {last_name} {age} {gender}");
    }

    // let i = rows.iter().map(|r| format("{} - {}",r.get::<i64>,_>("id"),r.get::<String,_>("first_name")).<collect::Vec<String>>().join(",");
    //
    // }

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
