use std::path::PathBuf;

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Deserialize, Serialize)]
pub struct Smov {
  pub id: i64,
  pub name: String,         //云端
  pub path: String,         //路径
  pub len: u64,             //大小
  pub created: i64,         //本地创建时间
  pub modified: i64,        //本地修改时间
  pub extension: String,    //拓展名
  pub format: String,       //格式化后名称
  pub release_time: String, //发行时间
  pub duration: i64,        //时长
  pub makers: String,       //商
  pub publishers: String,   //方
  pub series: String,       //系列
  pub directors: String,    //导演
  pub tags: Vec<String>,    //标签
  pub actors: Vec<String>,  //演员
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SmovSeek {
  pub id: i64,
  pub name: String,         //云端
  pub format: String,       //格式化后名称
  pub release_time: String, //发行时间
  pub duration: i32,        //时长
  pub publishers: String,   //方
  pub makers: String,       //商
  pub series: String,       //系列
  pub directors: String,    //导演
  pub tags: Vec<String>,    //标签
  pub actors: Vec<String>,  //演员
}

#[derive(Hash, Debug, Clone, Serialize, Deserialize)]
pub struct SmovFile {
  pub id: i64,
  pub realname: String, //当前的实际名称
  pub seekname: String,
  pub path: String,      //路径
  pub len: u64,          //大小
  pub created: i64,      //本地创建时间
  pub modified: i64,     //本地修改时间
  pub extension: String, //拓展名
  pub format: String,    //格式化后名称
}

#[derive(Hash, Debug, Clone, Deserialize, Serialize)]
pub struct SmovFileSeek {
  pub id: i64,
  pub realname: String,  //当前的实际名称
  pub path: String,      //路径
  pub extension: String, //拓展名
  pub format: String,    //格式化后名称
}

impl PartialEq for SmovFile {
  fn eq(&self, other: &Self) -> bool {
    (self.realname == other.realname)
      && (self.path == other.path)
      && (self.extension == other.extension)
  }
}

impl Eq for SmovFile {}

pub struct SMOVBOOK {
  _v: u64,
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

impl Smov {
  pub fn _insert_all(smov: Smov) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;

      tx.execute(
        "insert into maker(name) select ?1 where not exists (select * from maker where name= ?2)",
        params![smov.makers, smov.makers],
      )?;

      let maker: u64 = tx
        .query_row_and_then(
          "SELECT id from maker where name = ?1",
          params![smov.makers],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
        "insert into serie(name) select ?1 where not exists (select * from serie where name= ?2)",
        params![smov.series, smov.series],
      )
      .expect("插入出现错误？");

      let serie: u64 = tx
        .query_row_and_then(
          "SELECT id from serie where name = ?1",
          params![smov.series],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into director(name) select ?1 where not exists (select * from director where name= ?2)",
                params![smov.directors, smov.directors],
                )?;

      let director: u64 = tx
        .query_row_and_then(
          "SELECT id from director where name = ?1",
          params![smov.directors],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into publisher(name) select ?1 where not exists (select * from publisher where name= ?2)",
                params![smov.publishers, smov.publishers],
                )?;

      let publisher: u64 = tx
        .query_row_and_then(
          "SELECT id from publisher where name = ?1",
          params![smov.publishers],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      // 数据重复的错误 暂时不写
      // let flag :i32 = match tx
      // .query_row_and_then(
      //     "SELECT count(*) from smov where format = ?1",
      //     params![smov.makers],
      //     |row| row.get(0),
      // ){
      //     Ok(res) => {
      //         if res > 0 {
      //            return Err(rusqlite::Error::);
      //         }
      //         1
      //     }
      //     Err(e) => {
      //         return Err(e);
      //     }
      // };

      tx.execute(
                "insert into smov(name, path, len, created, modified, extension, format,publisher_id, makers_id, series_id, directors_id) select ?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11 where not exists(select * from smov where format = ?12)",
                params![smov.name, smov.path, smov.len, smov.created, smov.modified, smov.extension, smov.format,publisher,maker,serie,director,smov.format],
                ).expect("插入smov表出现错误");

      let smovid: u64 = tx
        .query_row_and_then(
          "SELECT id from smov where format = ?1",
          params![smov.format],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      for tag in smov.tags {
        tx.execute(
          "insert into tag(name) select ?1 where not exists (select * from tag where name= ?2)",
          params![tag, tag],
        )?;

        let tagid: u64 = tx
          .query_row_and_then("SELECT id from tag where name = ?1", params![tag], |row| {
            row.get(0)
          })
          .expect("查询出现错误");

        tx.execute(
          "insert into smov_tag(smov_id,tag_id) values(?1,?2)",
          params![smovid, tagid],
        )?;
      }

      for actor in smov.actors {
        tx.execute(
          "insert into actor(name) select ?1 where not exists (select * from actor where name= ?2)",
          params![actor, actor],
        )?;

        let actorid: u64 = tx
          .query_row_and_then(
            "SELECT id from actor where name = ?1",
            params![actor],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        tx.execute(
          "insert into smov_actor(smov_id,actor_id) values(?1,?2)",
          params![smovid, actorid],
        )?;
      }

      tx.commit().unwrap();

      Ok(())
    })
  }
}

impl SmovSeek {
  pub fn insert_by_path_name(smov: SmovSeek) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      tx.execute(
        "insert into maker(name) select ?1 where not exists (select * from maker where name= ?2)",
        params![smov.makers, smov.makers],
      )?;

      let maker: u64 = tx
        .query_row_and_then(
          "SELECT id from maker where name = ?1",
          params![smov.makers],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
        "insert into serie(name) select ?1 where not exists (select * from serie where name= ?2)",
        params![smov.series, smov.series],
      )
      .expect("插入出现错误？");

      let serie: u64 = tx
        .query_row_and_then(
          "SELECT id from serie where name = ?1",
          params![smov.series],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into director(name) select ?1 where not exists (select * from director where name= ?2)",
                params![smov.directors, smov.directors],
                )?;

      let director: u64 = tx
        .query_row_and_then(
          "SELECT id from director where name = ?1",
          params![smov.directors],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into publisher(name) select ?1 where not exists (select * from publisher where name= ?2)",
                params![smov.publishers, smov.publishers],
                )?;

      let publisher: u64 = tx
        .query_row_and_then(
          "SELECT id from publisher where name = ?1",
          params![smov.publishers],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      // 数据重复的错误 暂时不写
      // let flag :i32 = match tx
      // .query_row_and_then(
      //     "SELECT count(*) from smov where format = ?1",
      //     params![smov.makers],
      //     |row| row.get(0),
      // ){
      //     Ok(res) => {
      //         if res > 0 {
      //            return Err(rusqlite::Error::);
      //         }
      //         1
      //     }
      //     Err(e) => {
      //         return Err(e);
      //     }
      // };

      tx.execute(
                "update smov set name = ?1 ,makers_id =?2,series_id = ?3,directors_id =?4 , 
                publisher_id = ?5,duration = ?6,release_time = ?7 , is_retrieve = ?8 where id = ?9;",
                params![smov.name,maker,serie,director,publisher,smov.duration,smov.release_time,1,smov.id],
                ).expect("插入smov表出现错误");

      for tag in smov.tags {
        tx.execute(
          "insert into tag(name) select ?1 where not exists (select * from tag where name= ?2)",
          params![tag, tag],
        )?;

        let tagid: u64 = tx
          .query_row_and_then("SELECT id from tag where name = ?1", params![tag], |row| {
            row.get(0)
          })
          .expect("查询出现错误");

        tx.execute(
          "insert into smov_tag(smov_id,tag_id) values(?1,?2)",
          params![smov.id, tagid],
        )?;
      }

      for actor in smov.actors {
        tx.execute(
          "insert into actor(name) select ?1 where not exists (select * from actor where name= ?2)",
          params![actor, actor],
        )?;

        let actorid: u64 = tx
          .query_row_and_then(
            "SELECT id from actor where name = ?1",
            params![actor],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        tx.execute(
          "insert into smov_actor(smov_id,actor_id) values(?1,?2)",
          params![smov.id, actorid],
        )?;
      }

      tx.commit().unwrap();

      Ok(())
    })
  }
}

impl SmovFile {
  pub fn insert_file_data(smov_file: &Vec<&SmovFile>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;

      for y in smov_file {
        let format = crate::util::smov_format::SmovName::format_smov_name(&y.realname);
        tx.execute(
            "insert into smov(realname, path, len, created, modified, extension, format,seekname) select ?1,?2,?3,?4,?5,?6,?7,?8 where not exists(select * from smov where realname = ?9 and path = ?10)",
            params![y.realname,y.path,y.len,y.created,y.modified,y.extension,format,y.realname,y.realname,y.path],
            ).expect("插入smov表出现错误");
      }

      tx.commit().unwrap();

      Ok(())
    })
  }

  pub fn query_db_file_unid() -> Result<Vec<SmovFile>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt = conn
        .prepare("SELECT realname,seekname,path,len,created,modified,extension,format FROM smov")?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFile {
          id: 0,
          realname: row.get(0)?,      //当前的实际名称
          seekname: String::from(""), //当前的实际名称
          path: row.get(2)?,          //路径
          len: row.get(3)?,           //大小
          created: row.get(4)?,       //本地创建时间
          modified: row.get(5)?,      //本地修改时间
          extension: row.get(6)?,     //拓展名
          format: String::from(""),   //格式化后名称
        })
      })?;

      let mut res: Vec<SmovFile> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn query_by_id(id: &i64) -> Result<SmovFile, rusqlite::Error> {
    exec(|conn| {
      conn.query_row_and_then("SELECT realname,seekname,path,len,created,modified,extension,format FROM smov where id = ?1",
         params![id], |row| {
          Ok(SmovFile {
            id: 0,
            realname: row.get(0)?,      //当前的实际名称
            seekname: String::from(""), //当前的实际名称
            path: row.get(2)?,          //路径
            len: row.get(3)?,           //大小
            created: row.get(4)?,       //本地创建时间
            modified: row.get(5)?,      //本地修改时间
            extension: row.get(6)?,     //拓展名
            format: String::from(""),   //格式化后名称
          })
        })
    })
  }

  pub fn _query_unseek_db_file() -> Result<Vec<SmovFileSeek>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt =
        conn.prepare("SELECT id,realname,path,extension,format FROM smov where is_retrieve = 0")?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFileSeek {
          id: row.get(0)?,
          realname: row.get(1)?,  //当前的实际名称
          path: row.get(2)?,      //路径
          extension: row.get(3)?, //拓展名
          format: row.get(4)?,    //格式化后名称
        })
      })?;

      let mut res: Vec<SmovFileSeek> = Vec::new();
      for smov_file in smov_file_iter {
        res.push(smov_file.unwrap());
      }
      Ok(res)
    })
  }

  pub fn query_db_file_id_unseek() -> Result<Vec<SmovFile>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt =
            conn.prepare("SELECT id,realname,seekname,path,len,created,modified,extension,format FROM smov where is_retrieve = 0")?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFile {
          id: row.get(0)?,
          realname: row.get(1)?,  //当前的实际名称
          seekname: row.get(2)?,  //当前的实际名称
          path: row.get(3)?,      //路径
          len: row.get(4)?,       //大小
          created: row.get(5)?,   //本地创建时间
          modified: row.get(6)?,  //本地修改时间
          extension: row.get(7)?, //拓展名
          format: row.get(8)?,    //格式化后名称
        })
      })?;

      let mut res: Vec<SmovFile> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn _query_db_file_id() -> Result<Vec<SmovFile>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt = conn.prepare(
        "SELECT id,realname,seekname,path,len,created,modified,extension,format FROM smov",
      )?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(SmovFile {
          id: row.get(0)?,
          realname: row.get(1)?,  //当前的实际名称
          seekname: row.get(2)?,  //当前的实际名称
          path: row.get(3)?,      //路径
          len: row.get(4)?,       //大小
          created: row.get(5)?,   //本地创建时间
          modified: row.get(6)?,  //本地修改时间
          extension: row.get(7)?, //拓展名
          format: row.get(8)?,    //格式化后名称
        })
      })?;

      let mut res: Vec<SmovFile> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn update_seekname(id: i32, seek_name: String) -> Result<usize, rusqlite::Error> {
    exec(|conn| {
      let format = crate::util::smov_format::SmovName::format_smov_name(&seek_name);
      let update_size = conn
        .execute(
          "update smov set seekname = ?1 ,format = ?2 where id = ?3",
          params![seek_name, format, id],
        )
        .expect("更新出现错误");

      Ok(update_size)
    })
  }

  pub fn update_path_name(id : &i64,realname :String, path:String) -> Result<usize, rusqlite::Error>{
    exec(|conn| {

      let update_size = conn
        .execute(
          "update smov set realname = ?1 ,path = ?2 where id = ?3",
          params![realname, path, id],
        )
        .expect("更新出现错误");

      Ok(update_size)
    })
  }
}

impl SMOVBOOK {
  pub fn init() -> Result<()> {
    exec(|conn| {
      conn
        .execute(
          "create table if not exists smov
            (
                id           integer primary key autoincrement,
                name         TEXT,
                realname     TEXT,
                seekname     TEXT,
                path         TEXT,
                len          integer,
                created      integer,
                modified     integer,
                extension    TEXT,
                format       TEXT,
                release_time TEXT,
                duration     integer,
                publisher_id integer,
                makers_id    integer              Null,
                series_id    integer              Null,
                directors_id integer              Null,
                is_retrieve  TINYINT(1) Default 0 Null,
                is_active    TINYINT(1) Default 0 Null
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists actor
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists publisher
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists director
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists maker
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists serie
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists tag
            (
                id   integer primary key autoincrement,
                name TEXT
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists smov_actor
            (
                id       integer primary key autoincrement,
                smov_id  integer,
                actor_id integer
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists smov_tag
            (
                id      integer primary key autoincrement,
                smov_id integer,
                tag_id  integer
            )",
          [],
        )
        .unwrap();

      conn
        .execute(
          "Create Table if not exists sys_folder
            (
                id      integer primary key autoincrement,
                path    TEXT
            )",
          [],
        )
        .unwrap();
      Ok(())
    })
  }
}

impl SmovFileSeek {
  pub fn _seek2db(_smov_seek: &SmovSeek, _smov_file_seek: &SmovFileSeek) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      tx.commit().unwrap();
      Ok(())
    })
  }
}
