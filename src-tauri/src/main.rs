#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod cmd;
mod model;
mod response;
mod serve;
mod util;


use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;

#[tokio::main]
async fn main() {
    model::smov::SMOVBOOK::init().expect("数据库初始化出现错误");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏界面");
    let quit = CustomMenuItem::new("quit".to_string(), "退出软件");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a left click");
                let win = app.get_window("main").unwrap();
                win.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
             cmd::tauri_cmd::log_operation,
             cmd::tauri_cmd::perform_request,
             cmd::cmd::query_unretrieved,
             cmd::cmd::query_new_file_todb,
             cmd::cmd::update_seekname,
             cmd::cmd::insert_folder,
             cmd::cmd::query_folder,
             cmd::cmd::retrieve_data,
             cmd::tauri_cmd::open_folder_select
     ])
         .run(tauri::generate_context!())
         .expect("error while running tauri application")
        ;
}