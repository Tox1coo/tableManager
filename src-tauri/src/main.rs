// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use log::debug;
// use serde::{Deserialize, Serialize};
// use surrealdb::engine::remote::ws::Ws;
// use surrealdb::opt::auth::Root;
// use surrealdb::sql::Thing;
// use surrealdb::Surreal;



 fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");


}


// #[tokio::main]
// async fn connectdb() -> surrealdb::Result<()> {
//   let db = Surreal::new::<Ws>("http://127.0.0.1:5173/").await?;
//
//
//   // Signin as a namespace, database, or root user
//   db.signin(Root {
//     username: "Nastya",
//     password: "Nastya",
//   })
//       .await?;
//   // Select a specific namespace / database
//   db.use_ns("table").use_db("tables").await?;
//   // Create a new person with a random id
//
//   dbg!(&db);
//   println!("{}", "heloo");
//   Ok(())
// }