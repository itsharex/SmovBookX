use serde::Deserialize;
use std::fs::{write, File};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use tracing::info;
use window_shadows::set_shadow;
use window_vibrancy::{
  apply_acrylic, apply_blur, apply_mica, clear_acrylic, clear_blur, clear_mica,
};

use tauri::api::dialog;
use tauri::{command, Manager, Window, WindowUrl};

extern crate toml;
use crate::app::Conf;
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
pub async fn create_new_window(label: String, url: String, window: Window) {
  match window.get_window(&label) {
    Some(win) => set_focus(label, win),
    None => {
      println!("{}", &label.clone());
      Window::builder(
        &window,
        &label.clone(),
        WindowUrl::External(format!("http://localhost:3000/{}", url).parse().unwrap()),
      )
      .focus()
      .title(label.clone())
      .center()
      .min_inner_size(800.0, 600.0)
      .decorations(false)
      .build()
      .unwrap();
    }
  };
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
pub fn test() {
  info!(target: "frontend_log",message = "test msg" );
}

///这里到时候要做数据库式的配置修改 定位位置后修改那个位置的数据 需要改bug 这里有一步锁死了
#[command]
pub fn update_tidy_folder(path: String) {
  let app = &mut crate::app::APP.lock();
  app.conf.tidy_folder = PathBuf::from(&path);
  let to_path = app.app_dir.join("conf.toml");
  let a = Conf {
    tidy_folder: PathBuf::from(&path),
    thread: app.conf.thread,
  };
  if let Ok(_) = File::create(&to_path) {
    //写入一个数据
    let c = toml::to_string(&a).unwrap();
    write(&to_path, c).unwrap();
  }
}

#[command]
pub fn set_focus(label: String, window: Window) {
  match window.get_window(&label) {
    Some(win) => {
      #[cfg(any(target_os = "windows", target_os = "macos"))]
      set_shadow(&win, true).unwrap();
    }
    None => {}
  };

  window
    .emit_all(format!("{}_single", label).as_str(), "")
    .unwrap();
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
#[command]
#[inline]
pub fn set_style(effect: String, label: String, window: Window) {
  match window.get_window(&label) {
    Some(window) => {
      
      set_shadow(&window, true).unwrap();
      clear_blur(&window).unwrap();
      clear_acrylic(&window).unwrap();
      clear_mica(&window).unwrap();
      println!("{}",effect);
      match effect.as_str() {
        "blur" => apply_blur(&window,Some((238, 238, 244, 100))).unwrap(),
        "acrylic" => apply_acrylic(&window,Some((238, 238, 244, 100))).unwrap(),
        "mica" => apply_mica(&window).unwrap(),
        _ => (),
      };
    }
    None => {}
  };
}

pub type MaybeString = Option<String>;
