use std::{path::PathBuf, time::Duration};

use rusqlite::{params, Connection, Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Deserialize, Serialize)]
pub struct Smov {
  pub id: i64,
  pub name: String, //云端
  pub path: String, //路径
  pub realname: String,
  pub len: u64,             //大小
  pub created: i64,         //本地创建时间
  pub modified: i64,        //本地修改时间
  pub extension: String,    //拓展名
  pub format: String,       //格式化后名称
  pub release_time: String, //发行时间
  pub duration: i64,        //时长
  pub maker: String,        //商
  pub maker_id: i64,        //商
  pub publisher: String,    //方
  pub publisher_id: i64,    //方
  pub serie: String,        //系列
  pub serie_id: i64,        //系列
  pub director: String,     //导演
  pub director_id: i64,     //导演
  pub tags: Vec<Tag>,       //标签
  pub actors: Vec<Actor>,   //演员
  pub isch: bool,
  pub thumbs_img: String,
  pub main_img: String,
  pub detail_img: Vec<String>,
}

#[derive(Hash, Debug, Deserialize, Serialize)]
pub struct Tag {
  id: i64,
  name: String,
}

#[derive(Hash, Debug, Deserialize, Serialize)]
pub struct Actor {
  id: i64,
  name: String,
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
  pub isch: i32,
  pub is_active: i32,
}

#[derive(Hash, Debug, Clone, Deserialize, Serialize)]
pub struct SmovFileSeek {
  pub id: i64,
  pub realname: String,  //当前的实际名称
  pub path: String,      //路径
  pub extension: String, //拓展名
  pub format: String,    //格式化后名称
}

#[derive(Hash, Debug, Deserialize, Serialize, Clone)]
pub struct RetrievingSmov {
  pub id: i64,
  pub smov_id: i64,
  pub seek_name: String,
  pub status: i32,
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
  conn.busy_timeout(Duration::new(15, 0))?;
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
        params![smov.maker, smov.maker],
      )?;

      let maker: u64 = tx
        .query_row_and_then(
          "SELECT id from maker where name = ?1",
          params![smov.maker],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
        "insert into serie(name) select ?1 where not exists (select * from serie where name= ?2)",
        params![smov.serie, smov.serie],
      )
      .expect("插入出现错误？");

      let serie: u64 = tx
        .query_row_and_then(
          "SELECT id from serie where name = ?1",
          params![smov.serie],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into director(name) select ?1 where not exists (select * from director where name= ?2)",
                params![smov.director, smov.director],
                )?;

      let director: u64 = tx
        .query_row_and_then(
          "SELECT id from director where name = ?1",
          params![smov.director],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into publisher(name) select ?1 where not exists (select * from publisher where name= ?2)",
                params![smov.publisher, smov.publisher],
                )?;

      let publisher: u64 = tx
        .query_row_and_then(
          "SELECT id from publisher where name = ?1",
          params![smov.publisher],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      tx.execute(
                "insert into smov(name, path, len, created, modified, extension, format,publisher_id, makers_id, series_id, directors_id) select ?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11 where not exists(select * from smov where format = ?12)",
                params![smov.name, smov.path, smov.len, smov.created, smov.modified, smov.extension, smov.format,publisher,maker,serie,director,smov.format],
                ).expect("插入smov表出现错误");

      let _smovid: u64 = tx
        .query_row_and_then(
          "SELECT id from smov where format = ?1",
          params![smov.format],
          |row| row.get(0),
        )
        .expect("查询出现错误");

      //插入tag 暂时没用问题不大
      // for tag in smov.tags {
      //   tx.execute(
      //     "insert into tag(name) select ?1 where not exists (select * from tag where name= ?2)",
      //     params![tag, tag],
      //   )?;

      //   let tagid: u64 = tx
      //     .query_row_and_then("SELECT id from tag where name = ?1", params![tag], |row| {
      //       row.get(0)
      //     })
      //     .expect("查询出现错误");

      //   tx.execute(
      //     "insert into smov_tag(smov_id,tag_id) values(?1,?2)",
      //     params![smovid, tagid],
      //   )?;
      // }

      //插入演员暂时没用
      // for actor in smov.actors {
      //   tx.execute(
      //     "insert into actor(name) select ?1 where not exists (select * from actor where name= ?2)",
      //     params![actor, actor],
      //   )?;

      //   let actorid: u64 = tx
      //     .query_row_and_then(
      //       "SELECT id from actor where name = ?1",
      //       params![actor],
      //       |row| row.get(0),
      //     )
      //     .expect("查询出现错误");

      //   tx.execute(
      //     "insert into smov_actor(smov_id,actor_id) values(?1,?2)",
      //     params![smovid, actorid],
      //   )?;
      // }
      tx.commit().unwrap();

      Ok(())
    })
  }
  pub fn get_all_smov() -> Result<Vec<Smov>> {
    exec(|conn| {
      let tx = conn.transaction()?;
      let mut stmt = tx.prepare(
        "select id, name, path, realname, len, created, modified, extension, 
        format, release_time, duration,makers_id, publisher_id, series_id, directors_id, 
         isch from smov where is_retrieve = 1",
      )?;
      let smov_iter = stmt.query_map([], |row| {
        Ok(Smov {
          id: row.get(0)?,
          name: row.get(1)?,
          path: row.get(2)?,
          realname: row.get(3)?,
          len: row.get(4)?,
          created: row.get(5)?,
          modified: row.get(6)?,
          extension: row.get(7)?,
          format: row.get(8)?,
          release_time: row.get(9)?,
          duration: row.get(10)?,
          maker: String::from(""),
          maker_id: row.get(11)?,
          publisher: String::from(""),
          publisher_id: row.get(12)?,
          serie: String::from(""),
          serie_id: row.get(13)?,
          director: String::from(""),
          director_id: row.get(14)?,
          tags: Vec::new(),
          actors: Vec::new(),
          isch: row.get(15)?,
          thumbs_img: String::from(""),
          main_img: String::from(""),
          detail_img: Vec::new(),
        })
      })?;
      let mut smov_list: Vec<Smov> = Vec::new();

      for smov_s in smov_iter {
        let mut smov = smov_s.expect("序列化错误");
        smov.maker = tx
          .query_row_and_then(
            "SELECT name from maker where id = ?1",
            params![&smov.maker_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.publisher = tx
          .query_row_and_then(
            "SELECT name from publisher where id = ?1",
            params![&smov.publisher_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.serie = tx
          .query_row_and_then(
            "SELECT name from serie where id = ?1",
            params![&smov.serie_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        smov.director = tx
          .query_row_and_then(
            "SELECT name from director where id = ?1",
            params![&smov.director_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        let mut stmt = tx.prepare(
          "select tag.id,tag.name from tag,smov_tag where tag.id = smov_tag.tag_id and smov_tag.smov_id = ?1",
        )?;

        let tag_iter = stmt.query_map([smov.id], |row| {
          Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
          })
        })?;

        for tag in tag_iter {
          let tag1 = tag.unwrap();
          smov.tags.push(tag1);
        }

        let mut stmt = tx.prepare(
          "select actor.id,actor.name from actor,smov_actor where actor.id = smov_actor.actor_id and smov_actor.smov_id = ?1",
        )?;

        let actor_iter = stmt.query_map([smov.id], |row| {
          Ok(Actor {
            id: row.get(0)?,
            name: row.get(1)?,
          })
        })?;

        for actor in actor_iter {
          let actor1 = actor.unwrap();
          smov.actors.push(actor1);
        }

        smov_list.push(smov);
      }
      Ok(smov_list)
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
            "insert into smov(realname, path, len, created, modified, extension, format,seekname,isch) select ?1,?2,?3,?4,?5,?6,?7,?8,?9 where not exists(select * from smov where realname = ?10 and path = ?11)",
            params![y.realname,y.path,y.len,y.created,y.modified,y.extension,format,y.realname,y.isch,y.realname,y.path],
            ).expect("插入smov表出现错误");
      }

      tx.commit().unwrap();

      Ok(())
    })
  }

  //正常的标准写法！
  pub fn disable(id: Vec<i64>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      for y in id {
        match tx.execute("update smov set is_active = 0 where id = ?1", params![y]) {
          Ok(_) => {}
          Err(err) => return Err(err),
        };
      }
      tx.commit()
    })
  }

  pub fn change_active_status(id: i64, status: i32) -> Result<()> {
    exec(|conn| {
      match conn.execute(
        "update smov set is_active = ?1 where id = ?2",
        params![status, id],
      ) {
        Ok(_) => Ok(()),
        Err(err) => return Err(err),
      }
    })
  }

  pub fn query_db_file_unid() -> Result<Vec<SmovFile>, rusqlite::Error> {
    exec(|conn| {
      let mut stmt = conn.prepare(
        "SELECT realname,seekname,path,len,created,modified,extension,format,isch FROM smov",
      )?;
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
          isch: row.get(8)?,
          is_active: 0,
        })
      })?;

      let mut res: Vec<SmovFile> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.expect("出现错误");
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn query_by_id(id: &i64) -> Result<SmovFile, rusqlite::Error> {
    exec(|conn| {
      conn.query_row_and_then("SELECT realname,seekname,path,len,created,modified,extension,format,isch,is_active FROM smov where id = ?1",
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
            isch: row.get(8)?,
            is_active:row.get(9)?,
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
      let mut stmt = conn.prepare(
        "SELECT id,realname, seekname,path,len,created,modified,extension,format,isch,is_active
            FROM smov
            where is_retrieve = 0 and is_active=1
              and not exists(select 1 from seek_queue where smov_id = smov.id)",
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
          isch: row.get(9)?,
          is_active: row.get(10)?,
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
        "SELECT id,realname,seekname,path,len,created,modified,extension,format,isch,is_active FROM smov",
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
          isch: row.get(9)?,
          is_active: row.get(10)?,
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

  pub fn update_path_name(
    id: &i64,
    realname: String,
    path: String,
  ) -> Result<usize, rusqlite::Error> {
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

  pub fn delete_smov(id: Vec<i64>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      for y in id {
        match tx.execute("delete from smov where id = ?1", params![y]) {
          Ok(_) => {}
          Err(err) => {
            tx.rollback();
            return Err(err);
          }
        };
      }
      tx.commit()
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
                is_active    TINYINT(1) Default 1 Null,
                isch         TINYINT(1) Default 0 Null
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

      conn
        .execute(
          "create Table if not exists seek_queue
          (
              id      integer primary key autoincrement,
              smov_id integer,
              seekName TEXT,
              status  tinyint(1) default 0
          )",
          [],
        )
        .unwrap();

      Ok(())
    })
  }
}

impl RetrievingSmov {
  // pub fn change_seek_status(self:Self, )
}

impl SmovFileSeek {
  pub fn change_seek_status(smov: &mut Vec<RetrievingSmov>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;

      for y in smov {
        tx.execute(
          "insert into seek_queue(smov_id, seekName) values (?1,?2)",
          params![y.smov_id, y.seek_name],
        )
        .expect("添加索引队列出现错误！");

        y.id = tx
          .query_row_and_then(
            "SELECT id from seek_queue where smov_id = ?1",
            params![&y.smov_id],
            |row| row.get(0),
          )
          .expect("查询出现错误");

        y.status = 0;
      }

      tx.commit().expect("添加索引队列出现错误！");

      Ok(())
    })
  }

  pub fn get_seek_smov() -> Result<Vec<RetrievingSmov>> {
    exec(|conn| {
      let mut stmt = conn.prepare("SELECT id,smov_id,seekName,status FROM seek_queue")?;
      let smov_file_iter = stmt.query_map([], |row| {
        Ok(RetrievingSmov {
          id: row.get(0)?,
          smov_id: row.get(1)?,
          seek_name: row.get(2)?, //当前的实际名称
          status: row.get(3)?,
        })
      })?;

      let mut res: Vec<RetrievingSmov> = Vec::new();

      for smov_file in smov_file_iter {
        let s = smov_file.unwrap();
        res.push(s);
      }

      Ok(res)
    })
  }

  pub fn remove_smov_seek_status(id: Vec<i64>) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      for y in id {
        match tx.execute("delete from seek_queue where id = ?1", params![y]) {
          Ok(_) => {}
          Err(err) => return Err(err),
        };
      }
      tx.commit()
    })
  }

  pub fn change_status(id: i64, status: i64) -> Result<()> {
    exec(|conn| {
      let tx = conn.transaction()?;
      match tx.execute(
        "update seek_queue set status = ?1 where id = ?2",
        params![status, id],
      ) {
        Ok(_) => match tx.commit() {
          Ok(_) => Ok(()),
          Err(_) => return Err(Error::ExecuteReturnedResults),
        },
        Err(_) => return Err(Error::ExecuteReturnedResults),
      }
    })
  }
}
