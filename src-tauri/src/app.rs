/*! # app初始化相关
> app 工作目录初始化
> 配置初始化等
 */
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[cfg(not(target_os = "windows"))]
use std::io::{Read, Write};
#[cfg(target_os = "windows")]
use std::ptr::null;
use std::{
  fs::{create_dir_all, write, File, OpenOptions},
  io::Read,
  path::{Path, PathBuf},
  result::Result::Ok,
  sync::Arc,
};
use tauri::{
  command, AppHandle, CustomMenuItem, Event, Manager, Menu, SystemTray, SystemTrayEvent, Window,
  WindowMenuEvent, Wry,
};
extern crate toml;

#[cfg(not(target_os = "windows"))]
use tauri::{MenuItem, Submenu};
#[cfg(target_os = "windows")]
use tauri::{SystemTrayMenu, SystemTrayMenuItem};
use tokio::net::UdpSocket;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};
#[cfg(target_os = "windows")]
use windows::Win32::{
  Foundation::{GetLastError, WIN32_ERROR},
  System::Threading::{CreateMutexW, OpenMutexW},
};
#[cfg(target_os = "windows")]
use winreg::enums::*;
#[cfg(target_os = "windows")]
use winreg::RegKey;

use tracing::info;
use tracing_subscriber::{
  filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};

lazy_static! {
  pub static ref APP: Mutex<App> = Mutex::new(App::new());
  pub static ref CONF: Mutex<Conf> = Mutex::new(Conf::new());
}

///初始化app文件夹
pub fn init_app_dir() -> bool {
  if !Path::new(&crate::app::APP.lock().app_dir).exists() {
    if let Ok(_) = create_dir_all(&crate::app::APP.lock().app_dir) {
      return true;
    }
    return false;
  }
  true
}

pub fn init_app_conf() -> bool {
  if !Path::new(&crate::app::APP.lock().app_dir.join("conf.toml")).exists() {
    let path = &crate::app::APP.lock().app_dir.join("conf.toml");
    if let Ok(_) = File::create(path) {
      //写入一个数据
      let a = Conf {
        tidy_folder: PathBuf::new(),
      };
      let c = toml::to_string(&a).unwrap();
      write(path, c).unwrap();
      return true;
    }
    return false;
  }
  true
}

pub fn init_app_log() -> bool {
  // let format = fmt::format()
  //   .with_level(true) // don't include levels in formatted output
  //   .with_target(false) // don't include targets
  //   .with_thread_ids(false) // include the thread ID of the current thread
  //   .with_thread_names(true) // include the name of the current thread
  //   .compact(); // use the `Compact` formatting style.

  let file = &crate::app::APP.lock().app_dir.join("log");

  if !file.exists() {
    create_dir_all(file).expect("创建日志文件错误");
  }

  let file = match OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(file.join("app.log"))
  {
    Ok(file) => file,
    Err(error) => panic!("Error: {:?}", error),
  };

  let stdout_log = tracing_subscriber::fmt::layer().with_thread_names(true).with_target(false).with_file(false).pretty();

  let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

  let metrics_layer = /* ... */ filter::LevelFilter::INFO;

  tracing_subscriber::registry()
    .with(
      stdout_log
        .with_filter(filter::LevelFilter::INFO)
        .and_then(debug_log)
        .with_filter(filter::filter_fn(|metadata| {
          !metadata.target().starts_with("metrics") //不存在的
        })),
    )
    .with(metrics_layer.with_filter(filter::filter_fn(|metadata| {
      metadata.target().starts_with("metrics") //存在标签存在metrics的
    })))
    .init();


      // // This event will *only* be recorded by the metrics layer.
      info!(target: "metrics","watfuck");

      // // This event will only be seen by the debug log file layer:
      // tracing::debug!("this is a message, and part of a system of messages");
  
      // // This event will be seen by both the stdout log layer *and*
      // // the debug log file layer, but not by the metrics layer.
      // tracing::warn!("the message is a warning about danger!");

    info!("日志系统成功载入");
  true
}

/// app配置文件toml配置

/// app配置map
pub struct App {
  pub app_dir: PathBuf,
  // pub app_conf:PathBuf,
}

#[derive(Deserialize, Serialize)]
pub struct Conf {
  pub tidy_folder: PathBuf,
}

impl Conf {
  pub fn new() -> Conf {
    let conf = &crate::app::APP.lock().app_dir.join("conf.toml");
    let mut file = match File::open(conf) {
      Ok(f) => f,
      Err(e) => panic!("no such file {} exception:{}", conf.to_str().unwrap(), e),
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
      Ok(s) => s,
      Err(e) => panic!("Error Reading file: {}", e),
    };

    let config: Conf = toml::from_str(&str_val).unwrap();

    config
  }
}

impl App {
  pub fn new() -> App {
    let cfg = tauri::Config::default();
    match tauri::api::path::app_dir(&cfg) {
      None => App {
        app_dir: PathBuf::new(),
      },
      Some(p) => App {
        app_dir: p.join("smovbook"),
      },
    }
  }
}

/// 创建任务栏图标
#[cfg(target_os = "windows")]
pub fn create_try() -> SystemTray {
  let quit = CustomMenuItem::new("quit".to_string(), "退出");
  let set = CustomMenuItem::new("set".to_string(), "设置");
  let tray_menu = SystemTrayMenu::new()
    .add_item(set)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);
  SystemTray::new().with_menu(tray_menu)
}

/// 创建任务栏图标
#[cfg(not(target_os = "windows"))]
pub fn create_try() -> SystemTray {
  SystemTray::new()
}

/// 系统菜单
#[cfg(not(target_os = "windows"))]
pub fn create_app_menu() -> Menu {
  let quit = CustomMenuItem::new("set".to_string(), "设置");
  let close = CustomMenuItem::new("exit".to_string(), "退出");
  let submenu = Submenu::new("软件", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_submenu(submenu);
  menu
}

/// 系统菜单
#[cfg(target_os = "windows")]
pub fn create_app_menu() -> Menu {
  Menu::new()
}

pub fn handle_event_app_menu_event(event: WindowMenuEvent<Wry>) {
  match event.menu_item_id() {
    "exit" => {
      std::process::exit(0);
    }
    "set" => {
      event.window().emit("set", "").unwrap();
    }
    _ => {
      print!("at handle_event_app_menu_event")
    }
  }
}

/// 任务栏图标点击事件
pub fn handle_system_tray_event(app: &AppHandle<Wry>, e: SystemTrayEvent) {
  match e {
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "quit" => {
        std::process::exit(0);
      }
      "set" => {
        app.get_window("main").unwrap().emit("set", "").unwrap();
      }
      _ => {}
    },
    SystemTrayEvent::LeftClick { .. } => {
      if let Some(window) = app.get_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
        info!("handle_system_tray_event at here?");
      }
    }
    _ => {}
  }
}

/// 监听app事件
pub fn handle_app_event(app_handle: &AppHandle<Wry>, event: Event) {
  match event {
    Event::CloseRequested { label, api, .. } => {
      if label == "main" {
        let app_handle = app_handle.clone();
        app_handle.get_window(&label).unwrap().hide().unwrap();
        // use the exposed close api, and prevent the event loop to close
        api.prevent_close();
      }
    }
    _ => {}
  }
}

/// 创建pid文件
#[cfg(not(target_os = "windows"))]
pub fn crete_pid_file() {
  let pid = std::path::PathBuf::from(&APP.lock().app_dir).join("app.pid");
  let id = std::process::id();
  let mut fd = std::fs::File::create(pid).unwrap();
  let _ = fd.write_all(format!("{}", id).as_bytes()).unwrap();
}

/// 锁定单例模式 windows
#[cfg(target_os = "windows")]
pub fn lock_single() {
  unsafe {
    let _ = OpenMutexW(0, true, "SmovBook@leri");
    let WIN32_ERROR(code) = GetLastError();
    if code == 2 {
      // 创建锁
      let _ = CreateMutexW(null(), true, "SmovBook@leri");
    } else {
      // 已经存在了，退出
      send_wake_up();
      std::process::exit(0);
    }
  }
}

/// 锁定单例模式 linux
#[cfg(not(target_os = "windows"))]
pub fn lock_single() {
  // check pid file is exists?
  let pid = std::path::PathBuf::from(&APP.lock().app_dir).join("app.pid");
  // pid is not exists? create pid file an start app
  if !pid.exists() {
    return;
  }
  // pid is exists? check the app is running?
  let fd = std::fs::File::open(pid);
  let mut data = vec![];
  fd.unwrap().read_to_end(&mut data).unwrap();
  let fd = String::from_utf8(data).unwrap().parse().unwrap();
  unsafe {
    // send a signal to check process is running?
    let status = libc::kill(fd, 0);
    // running ?
    if status == 0 {
      send_wake_up();
      std::process::exit(0);
    } else {
      // not running create a pid file
      crete_pid_file()
    }
  }
}

/// 发送拉起请求
fn send_wake_up() {
  tauri::async_runtime::block_on(async {
    let res = UdpSocket::bind("127.0.0.1:24253").await.unwrap();
    let mut data = [0u8; 16];
    for i in 0..16 {
      data[i] = 1 as u8
    }
    res.send_to(&data, "127.0.0.1:24254").await.unwrap();
  });
}
#[cfg(target_os = "windows")]
fn open_reg_key() -> std::io::Result<()> {
  // first find current user reg table
  let current_key = RegKey::predef(HKEY_CURRENT_USER);
  let wv2 = current_key.open_subkey(
    "Software\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
  );
  if let Ok(key) = wv2 {
    let res: std::io::Result<String> = key.get_value("pv");
    if let Ok(_) = res {
      return Ok(());
    }
  };
  // then find all account reg table
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let w2 = hklm.open_subkey(
    "SOFTWARE\\WOW6432Node\\Microsoft\\EdgeUpdate\\Clients\\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
  )?;
  let _: String = w2.get_value("pv")?;
  Ok(())
}

//windows下检查是否安装了WebView2
#[cfg(target_os = "windows")]
pub fn webview2_is_installed() {
  if let Err(_) = open_reg_key() {
    unsafe {
      MessageBoxW(
        None,
        "WebView2运行时未找到，点击确定按钮去安装吧！",
        "出错啦！",
        MB_OK,
      );
      let _ = tauri::api::shell::open(
        "https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/#download-section"
          .to_string(),
        None,
      );
      std::process::exit(0);
    }
  };
}

#[command]
pub async fn listen_single(window: Window) {
  let _: tauri::async_runtime::JoinHandle<anyhow::Result<(), anyhow::Error>> =
    tauri::async_runtime::spawn(async move {
      let socket = UdpSocket::bind("127.0.0.1:24254").await.expect("连接失败");
      loop {
        let mut buf = [0; 32];
        let (size, _) = socket.recv_from(&mut buf).await.expect("出现错误");
        if size != 16 {
          return Ok(());
        };
        // check status
        let mut status = true;
        for item in &buf[0..size] {
          if *item as i32 != 1 {
            status = false;
            break;
          }
        }
        if status {
          let _ = window.emit_all("single", "");
        };
      }
    });
}
