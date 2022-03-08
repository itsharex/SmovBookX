use std::thread;

use crate::model::folder::Folder;
use crate::model::smov::RetrievingSmov;
use crate::model::smov::Smov;
use crate::model::smov::SmovFile;
use crate::model::smov::SmovFileSeek;
use crate::response::response::Response;
use crate::serve::smov;
use crate::serve::smov_file;
use crate::util::smov_format::SmovName;
use tauri::command;
use tauri::Manager;
use tauri::Window;
use tracing::info;

//检索新文件到数据库
#[command]
pub fn query_new_file_todb() -> Response<String> {
  Response::ok(smov_file::smov_file(), "检索成功")
}

#[command]
pub async fn retrieve_data(retrieving_smov: RetrievingSmov) -> Response<Option<i32>> {
  let format = SmovName::format_smov_name(&retrieving_smov.seek_name);
  let handle = match thread::Builder::new()
    .name(String::from(&retrieving_smov.seek_name))
    .spawn(move || {
      let msg = format!("开始检索{}", &retrieving_smov.seek_name);
      info!(target: "frontend_log",message = msg.as_str());
      let s = tauri::async_runtime::block_on(async move {
        let a = smov::retrieve_smov(format, retrieving_smov.smov_id).await;
        info!("{}", "线程检索结束");
        return a;
      });
      let msg = format!("检索结束{}", &retrieving_smov.seek_name);
      info!(target: "frontend_log",message = msg.as_str());
      return s;
    }) {
    Ok(res) => res,
    Err(e) => return Response::err(None, format!("{}", &e).as_str()),
  };

  // "线程创建错误" return Response::err(Some(1),

  let handle = handle.join();

  match handle {
    Ok(e) => match e {
      Ok(n) => match n {
        true => {
          SmovFileSeek::change_status(retrieving_smov.id, 1).expect("出现了一个错误");
          Response::new(200, Some(1), "success")
        }
        false => {
          SmovFileSeek::change_status(retrieving_smov.id, 2).expect("出现了一个错误");
          Response::new(404, Some(1), "not found")
        }
      },
      Err(err) => Response::err(None,format!("{}", &err).as_str() ),
    },
    Err(_e) => Response::err(None, "出现了一个我不会解析的错误！"),
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

#[command]
pub fn change_seek_status(smov: Vec<RetrievingSmov>, window: Window) -> Response<Option<bool>> {
  let mut to_smov = smov.clone();

  match SmovFileSeek::change_seek_status(&mut to_smov) {
    Ok(_) => {
      window
        .emit_to("seek", "addTask", &to_smov)
        .expect("向另一个窗口传送数据出现错误");
      return Response::new(200, Some(true), "success");
    }
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  };
}

#[command]
pub fn get_seek_smov() -> Response<Option<Vec<RetrievingSmov>>> {
  match SmovFileSeek::get_seek_smov() {
    Ok(e) => return Response::new(200, Some(e), "success"),
    Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
  };
}

#[command]
pub fn remove_smov_seek_status(id: i64) -> Response<Option<bool>> {
  match SmovFileSeek::remove_smov_seek_status(id) {
    Ok(_) => return Response::new(200, Some(true), "success"),
    Err(err) =>{
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str())
    } 
  };
}

#[command]
pub fn disable_smov(id :Vec<i64>) ->Response<Option<bool>>{
  match SmovFile::disable(id) {
    Ok(_) => return Response::new(200, Some(true), "success"),
    Err(err) =>{
      tracing::error!(message = format!("{}", err).as_str());
      return Response::new(300, None, format!("{}", err).as_str())
    } 
  };
}
