#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

mod app;
mod cmd;
mod model;
mod response;
mod serve;
mod util;

#[tokio::main]
async fn main() {
  app::lock_single();

  let _app = tauri::Builder::default()
    .setup(|_app| {
      if cfg!(target_os = "windows") {
        app::webview2_is_installed(_app);
      }
      if !app::init_app_dir() {
        panic!("工作目录初始化失败！");
      }
      if !app::init_app_log(_app) {
        panic!("日志系统初始化失败！");
      }
      model::smov::SMOVBOOK::init().expect("数据库初始化出现错误");
      Ok(())
    })
    .menu(app::create_app_menu())
    .on_menu_event(app::handle_event_app_menu_event)
    .system_tray(app::create_try())
    .on_system_tray_event(app::handle_system_tray_event)
    .invoke_handler(tauri::generate_handler![
      app::listen_single,
      cmd::tauri_cmd::log_operation,
      cmd::tauri_cmd::perform_request,
      cmd::cmd::query_unretrieved,
      cmd::cmd::query_new_file_todb,
      cmd::cmd::update_seekname,
      cmd::cmd::insert_folder,
      cmd::cmd::query_folder,
      cmd::cmd::retrieve_data,
      cmd::cmd::get_all_smov,
      cmd::cmd::change_seek_status,
      cmd::cmd::get_seek_smov,
      cmd::cmd::disable_smov,
      cmd::cmd::change_active_status,
      cmd::cmd::delete_smov,
      cmd::cmd::remove_smov_seek_status,
      cmd::cmd::get_smov_by_id,
      cmd::tauri_cmd::open_folder_select,
      cmd::tauri_cmd::test,
      cmd::tauri_cmd::open_in_explorer,
      cmd::tauri_cmd::update_tidy_folder,
      cmd::tauri_cmd::set_focus,
      cmd::tauri_cmd::create_new_window,
      cmd::tauri_cmd::set_style
    ])
    .build(tauri::generate_context!())
    .expect("error while running tauri application");

  _app.run(app::handle_app_event);
}
