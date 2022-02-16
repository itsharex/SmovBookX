use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, result::Result::Ok};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
  pub id: i64,
  pub path: String,
}

fn get_conn() -> Connection {
  let conn = Connection::open("SmovBook.db");
  if conn.is_ok() {
  } else {
    println!("连接失败:{:?}", conn.as_ref().err().unwrap().to_string());
  }
  conn.unwrap()
}

fn create_sqlite_connection() -> Result<Connection> {
  let database = PathBuf::from(&crate::app::APP.lock().app_dir).join("SmovBook.db");
  let conn = Connection::open(database)?;
  Ok(conn)
}
/// 封装一个方法，获取连接
pub fn exec<F, T>(func: F) -> Result<T>
where
  F: FnOnce(&mut Connection) -> Result<T>,
{
  match create_sqlite_connection() {
    Ok(mut conn) => func(&mut conn),
    Err(e) => Err(e),
  }
}

impl Folder {
  pub fn insert_folder(path: String) -> Result<i32, rusqlite::Error> {
   exec(|conn| { 
    conn.execute(
            "insert into sys_folder(path) select ?1 where not exists(select * from sys_folder where path = ?2)",
            params![path,path],
            ).expect("插入smov表出现错误");

    let folder_id: i32 = conn
      .query_row_and_then(
        "SELECT id from sys_folder where path = ?1",
        params![path],
        |row| row.get(0),
      )
      .expect("查询出现错误");

      Ok(folder_id)
    })
  }
  
  pub fn query_folder() -> Result<Vec<Folder>, rusqlite::Error> {
   exec(|conn| { 
    let mut stmt = conn.prepare("SELECT id,path FROM sys_folder")?;
    let folder_iter = stmt.query_map([], |row| {
      Ok(Folder {
        id: row.get(0)?,
        path: row.get(1)?,
      })
    })?;

    let mut res: Vec<Folder> = Vec::new();

    for smov_file in folder_iter {
      let s = smov_file.unwrap();
      res.push(s);
    }
    Ok(res)
  })
  }
}
