use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, result::Result::Ok};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub id: i64,
    pub path:String
}

// /// 创建数据库连接
// fn create_sqlite_connection() -> Result<Connection> {
//     let database = PathBuf::from(&crate::app::APP.lock().app_dir).join("app.db");
//     let conn = Connection::open(database)?;
//     Ok(conn)
// }
// /// 封装一个方法，获取连接
// pub fn exec<F, T>(func: F) -> Result<T>
// where
//     F: FnOnce(&mut Connection) -> Result<T>,
// {
//     match create_sqlite_connection() {
//         Ok(mut conn) => func(&mut conn),
//         Err(e) => Err(e),
//     }
// }

impl Folder {
    pub fn insert_folder(path:String) -> i32 {
        
        1
    } 
    
}