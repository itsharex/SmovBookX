use std::thread;

use crate::model::folder::Folder;
use crate::model::smov::Smov;
use crate::model::smov::SmovFile;
use crate::response::response::Response;
use crate::serve::smov;
use crate::serve::smov_file;
use crate::util::smov_format::SmovName;
use tauri::command;
use tracing::info;

//检索新文件到数据库
#[command]
pub fn query_new_file_todb() -> Response<String> {
  Response::ok(smov_file::smov_file(), "检索成功")
}

#[command]
pub async fn retrieve_data(seek_name: String, smov_id: i64) -> Response<Option<i32>> {
  let format = SmovName::format_smov_name(&seek_name);
  info!("开始检索{}", seek_name);
  let handle = thread::Builder::new()
    .name(seek_name)
    .spawn(move || {
      let s: bool = tauri::async_runtime::block_on(async move {
        let a = smov::retrieve_smov(format, smov_id).await.unwrap();
        info!("{}", "线程检索结束");
        return a;
      });
      return s;
    })
    .expect("线程创建错误")
    .join();

  match handle {
    Ok(e) => match e {
      true => Response::new(200, Some(1), "success"),
      false => Response::new(404, Some(1), "not found"),
    },
    Err(_e) => Response::err(Some(1), "出现未知错误"),
  }
}

//查找所有未被检索的数据
#[command]
pub fn query_unretrieved() -> Response<Option<Vec<SmovFile>>> {
  // info!("such information");
  match SmovFile::query_db_file_id_unseek() {
    Ok(e) => return Response::new(200, Some(e), "success"),
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

#[command]
pub fn get_all_smov() -> Response<Option<Vec<Smov>>> {
  match Smov::get_all_smov() {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  }
}
