#[macro_use]

extern crate diesel;
extern crate dotenvy;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::CoreInfo;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn write_core_info(conn: &mut MysqlConnection, seed: i32, session_num: i32, is_running: bool, body: String) -> CoreInfo {
  use crate::schema::core_infos;

  let id = 0;
  let new_core_info = CoreInfo { id, seed, session_num, is_running, body };

  diesel::insert_into(core_infos::table)
      .values(&new_core_info)
      .execute(conn)
      .expect("Error saving core info");

  core_infos::table.order(core_infos::id.desc()).first(conn).unwrap()
}
