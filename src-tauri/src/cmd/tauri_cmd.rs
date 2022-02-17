use serde::Deserialize;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;

use tauri::api::dialog;
use tauri::command;

use crate::response::response::Response;

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
    .arg(".") // <- Specify the directory you'd like to open.
    .spawn()
    .unwrap();
}

pub fn pathbuf_to_string(pathbuf: PathBuf) -> MaybeString {
  pathbuf.to_str().map(|st| String::from(st))
}

//测试
#[command]
pub fn test() -> String {
  let s = &crate::app::CONF.lock().tidy_folder.to_str().unwrap().to_string();
  println!("{}",s);
  s.clone()
}


pub type MaybeString = Option<String>;
