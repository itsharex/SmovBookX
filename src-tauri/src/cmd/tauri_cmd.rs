use serde::Deserialize;
use std::fs::{write, File};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;

use tauri::api::dialog;
use tauri::command;

extern crate toml;
use crate::app::Conf;
use crate::response::response::Response;
use crate::serve::file;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  // your custom commands
  // multiple arguments are allowed
  // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
  MyCustomCommand { argument: String },
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RequestBody {
  id: i32,
  name: String,
}

#[command]
pub fn log_operation(event: String, payload: Option<String>) {
  println!("{} {:?}", event, payload);
}

#[command]
pub fn perform_request(endpoint: String, body: RequestBody) -> String {
  println!("{} {:?}", endpoint, body);
  "message response".into()
}

#[command]
pub fn open_folder_select() -> Response<String> {
  let (sender, receiver) = channel::<PathBuf>();
  let a = dialog::FileDialogBuilder::new();
  a.pick_folder(move |path| {
    if path.is_some() {
      sender.send(path.unwrap()).unwrap()
    } else {
      drop(sender)
    }
  });

  let maybe_received = receiver.recv();
  if maybe_received.is_err() {
    return Response::err(String::from("No folder selected"), "No folder selected");
  }

  let foo = maybe_received.unwrap();
  let maybe_str = pathbuf_to_string(foo);
  if maybe_str.is_none() {
    return Response::err(String::from("Invalid folder path"), "Invalid folder path");
  }

  let selection_str = maybe_str.unwrap();

  Response::ok(selection_str, "成功")
}

//在资源管理器中打开
#[command]
pub fn open_in_explorer(path: String) {
  Command::new("explorer")
    .arg(path) // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

pub fn pathbuf_to_string(pathbuf: PathBuf) -> MaybeString {
  pathbuf.to_str().map(|st| String::from(st))
}

//测试
#[command]
pub fn test() -> String {
  let s = file::TidySmov{
    id : &4,
    name: &String::from("测试1")
  };
  s.tidy();

  String::from("测试")

}

///这里到时候要做数据库式的配置修改 定位位置后修改那个位置的数据
#[command]
pub fn update_tidy_folder(path: String) {
  let mut conf = crate::app::CONF.lock();
  conf.tidy_folder = PathBuf::from(&path);
  let to_path = &crate::app::APP.lock().app_dir.join("conf.toml");
  let a = Conf {
    tidy_folder: PathBuf::from(&path),
  };
  if let Ok(_) = File::create(to_path) {
    //写入一个数据
    let c = toml::to_string(&a).unwrap();
    write(to_path, c).unwrap();
  }
}

pub type MaybeString = Option<String>;
