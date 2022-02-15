use crate::model::folder::Folder;
use crate::model::smov::SmovFile;
use crate::response::response::Response;
use crate::serve::smov;
use crate::serve::smov_file;
use crate::util::smov_format::SmovName;
use tauri::command;

use std::thread;
use std::time::Duration;

//检索新文件到数据库
#[command]
pub fn query_new_file_todb() -> Response<String> {
  Response::ok(smov_file::smov_file(), "检索成功")
}

pub fn query_smov() {}

pub fn query_smov_list() {}

#[command]
pub async fn retrieve_data(seek_name: String, smov_id: i64) -> Response<Option<i32>> {
  //更新数据库的seekname
  let format = SmovName::format_smov_name(&seek_name);

  let handle = std::thread::spawn(move || {
    let s = tauri::async_runtime::block_on(async move {
      let a: Result<bool, anyhow::Error> = smov::get_test(format, smov_id).await;
      // InvokeResolver::new(window, String::from("123"), String::from("123"));
      //println!("{}", "线程检索结束");

      // Ok((1))
      //异步回调难以实现
    });
  })
  .join();
  // let s = handle.join().unwrap(); //等待子线程结束

  //   let _: tauri::async_runtime::JoinHandle<anyhow::Result<(), anyhow::Error>> =
  //     tauri::async_runtime::spawn(async move {
  //        let _: Result<bool, anyhow::Error> = smov::get_test(format, smov_id).await;
  //       Ok(())
  //   });

  //再来一个查询看看有没有查成功 直接返回的以后再说

  Response::new(200, Some(1), "success")
}

#[command]
pub async fn open_smov_win() -> i64 {
  thread::sleep(Duration::from_millis(10));
  1
}

//查找所有未被检索的数据
#[command]
pub fn query_unretrieved() -> Response<Option<Vec<SmovFile>>> {
  match SmovFile::query_db_file_id_unseek() {
    Ok(e) => return Response::new(200, Some(e), "success"), //return serde_json::to_string(&e).expect("序列化出现错误"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  };
}

#[command]
pub fn update_seekname(id: i32, seek_name: String) -> Response<Option<usize>> {
  match SmovFile::update_seekname(id, seek_name) {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  };
}

#[command]
pub fn insert_folder(path: String) -> Response<Option<i32>> {
  match Folder::insert_folder(path) {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}

#[command]
pub fn query_folder() -> Response<Option<Vec<Folder>>> {
  match Folder::query_folder() {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}
