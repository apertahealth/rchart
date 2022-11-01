#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use tauri::State;
// use sqlx::{sqlite::Sqlite, SqliteQueryResult, SqlitePool, migrate::MigrateDatabase};
// use sqlx::Error as sqERR;
// use tauri_plugin_sql::TauriSql;
// use sqlite::Error as sqERR;

// pub struct User {
//   fname: String,
// }

// #[derive(Debug)]
// pub enum UserErr {
//   DbErr(sqERR)
// }

// impl From<sqERR> for UserErr {
//   fn from(s: sqERR) -> Self {
//     UserErr::DbErr(s)
//   }
// }

// impl User {
//   pub fn add(&self, username: &str, password: &str) -> Result<(), UserErr> {
//     let connection = sqlite::open(&self.fname)?;
//     let mut db = connection.prepare("insert into users(username, password) values (?, ?);")?;
//     db.bind(1, username)?;
//     db.bind(2, password)?;
//     db.next()?;
//     Ok(())
//   }
// }

//#[tokio::main]
#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

// #[async_std::main]
fn main() {
  // let db_url = String::from("sqlite://sqlite.db");
  //   if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
  //       Sqlite::create_database(&db_url).await.unwrap();
  //       match cretea_schema(&db_url).await {
  //           Ok(_) => println!("Database created Sucessfully"),
  //           Err(e) => panic!("{}",e),
  //       }
  //   }
  //   let instances = SqlitePool::connect(&db_url).await.unwrap();
  //   let qry ="INSERT INTO settings (description) VALUES($1)";
  //   let result = sqlx::query(&qry).bind("testing").execute(&instances).await;
    
  //   instances.close().await;

  //   println!("{:?}", result);

  // open a pool object to connect to the database
  //sqlx::SqlitePool::connect

  // execute a query
  //sqlx::query(&qry).execute(&pool);

  //let pool = SqlitePoolOptions::new()
   // .max_connections(5)
    //.connect("sqlite://sqlite:password@localhost/test").await?;
  
    tauri::Builder::default()
    //.plugin(TauriSql::default())
    .manage(Counter(Default::default()))
    .invoke_handler(tauri::generate_handler![get_tasks])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// function to add ryan to db
// #[tauri::command]
// fn add_ryan() -> String {
//   let db = User {
//     fname: String::from("./data/sqlite.db"),
//   };

//   match db.add("ryan", "top_secret") {
//     Ok(_) => println!("Adding ryan was a success!"),
//     Err(UserErr::DbErr(ref err)) => println!(":( {:?}", err)
//   }
  
//   "Hello World!".to_string()
// }

// // grab tasks
#[tauri::command]
fn get_tasks() -> String {
  "Hi".to_string()
}

// async function
// async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> {
//   let pool = SqlitePool::connect(&db_url).await?;
//   let qry = 
//   "PRAGMA foreign_keys = ON ;
//   CREATE TABLE IF NOT EXISTS settings
//       (
//           settings_id             INTEGER PRIMARY KEY NOT NULL,
//           description             TEXT                NOT NULL,
//           created_on              DATETIME DEFAULT (datetime('now','localtime')),
//           updated_on              DATETIME DEFAULT (datetime('now','localtime')),
//           done                    BOOLEAN             NOT NULL DEFAULT 0
//       );    
//   CREATE TABLE IF NOT EXISTS project
//       (
//           project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
//           product_name                 TEXT ,
//           created_on                   DATETIME DEFAULT (datetime('now','localtime')),
//           updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
//           img_directory                TEXT NOT NULL,
//           out_directory                TEXT NOT NULL,
//           status                       TEXT NOT NULL,
//           settings_id                  INTEGER  NOT NULL DEFAULT 1,
//           FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
//       );";
//   let result = sqlx::query(&qry).execute(&pool).await;   
//   pool.close().await; 
//   return result;
// }