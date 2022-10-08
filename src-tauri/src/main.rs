#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

//use std::sync::{Arc, Mutex};
//use tauri::State;
// use sqlx::sqlite::SqlitePoolOptions;
// use sqlx::Error as sqERR;
// use tauri_plugin_sql::TauriSql;
use sqlite::Error as sqERR;

pub struct User {
  fname: String,
}

#[derive(Debug)]
pub enum UserErr {
  DbErr(sqERR)
}

impl From<sqERR> for UserErr {
  fn from(s: sqERR) -> Self {
    UserErr::DbErr(s)
  }
}

impl User {
  pub fn add(&self, username: &str, password: &str) -> Result<(), UserErr> {
    let connection = sqlite::open(&self.fname)?;
    let mut db = connection.prepare("insert into users(username, password) values (?, ?);")?;
    db.bind(1, username)?;
    db.bind(2, password)?;
    db.next()?;
    Ok(())
  }
}

//#[tokio::main]
//#[derive(Default)]
//struct Counter(Arc<Mutex<i32>>);
fn main() {
  

  // open a pool object to connect to the database
  //sqlx::SqlitePool::connect

  // execute a query
  //sqlx::query(&qry).execute(&pool);

  //let pool = SqlitePoolOptions::new()
  //  .max_connections(5)
  //  .connect("sqlite://sqlite:password@localhost/test").await?;

  tauri::Builder::default()
    //.plugin(TauriSql::default())
    //.manage(Counter(Default::default()))
    .invoke_handler(tauri::generate_handler![add_ryan])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// function to add ryan to db
#[tauri::command]
fn add_ryan() -> String {
  let db = User {
    fname: String::from("./data/sqlite.db"),
  };

  match db.add("ryan", "top_secret") {
    Ok(_) => println!("Adding ryan was a success!"),
    Err(UserErr::DbErr(ref err)) => println!(":( {:?}", err)
  }
  
  "Hello World!".to_string()
}

// grab tasks
#[tauri::command]
fn get_tasks() -> String {
  "Hi".to_string()
}