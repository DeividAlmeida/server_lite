use std::error::Error;
// use rusqlite::{ params };
use rand::Rng;

use crate::structs::publisher::Publisher;
mod connection; 

fn raffle() -> (usize, &'static str, i32) {
  const ORDER: [i32; 2] = [-1, 1];
  const ITEMS: [&str; 3] = ["type", "name", "updated_at"];
  
  let mut rng = rand::thread_rng();
  let item_index: usize = rng.gen_range(0..ITEMS.len());
  let order_index: usize = rng.gen_range(0..ORDER.len());
  
  (item_index, ITEMS[item_index], ORDER[order_index])
}

pub fn create_publisher(publisher:Publisher) -> Result<usize, Box<dyn Error>> {
  let conn = connection::sqlite().unwrap();
 let insert = conn.execute(
    "INSERT INTO publishers (name, type, gender, updated_at, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
    (publisher.name, publisher.r#type, publisher.gender, 1, 1),
  );

  match insert {
    Ok(value) =>
      Ok(value),
    Err(erro) =>
      Err(erro.into()),
  }
}

pub fn list_publisher() -> Result<String, Box<dyn Error>> {
  let conn = connection::sqlite().unwrap();
  
  let publishers: Vec<Publisher> = conn
  .prepare("SELECT * FROM publishers")?
  .query_map([], |row| {
      Ok(Publisher {
          id: row.get(0)?,
          name: row.get(1)?,
          r#type: row.get(2)?,
          gender: row.get_unwrap(3),
          amount: row.get_unwrap(4),
          active: row.get_unwrap(5),
          updated_at: row.get_unwrap(6),
          created_at: row.get_unwrap(7),
      })
  })?
  .filter_map(Result::ok)
  .collect();

  match serde_json::to_string(&publishers) {
    Ok(json) =>  Ok(json),
    Err(erro) =>  Err(erro.into()),
  }

}