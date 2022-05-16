/*! # app初始化相关
> app 工作目录初始化
> 配置初始化等
 */
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  net::{IpAddr, Ipv4Addr},
};
use toml::Value;

#[cfg(not(target_os = "windows"))]
use std::io::{Read, Write};
#[cfg(target_os = "windows")]
use std::ptr::null;
use std::{
  collections::BTreeMap,
  fs::{create_dir_all, write, File, OpenOptions},
  io::Read,
  path::PathBuf,
  result::Result::Ok,
  sync::Arc,
  thread,
};
use tauri::{
  command, AppHandle, CustomMenuItem, Manager, Menu, RunEvent, SystemTray, SystemTrayEvent, Window,
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
  pub static ref HFSCONFIG: Mutex<HfsConfig> = Mutex::new(HfsConfig::new());
}

struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
  fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
    self
      .0
      .insert(field.name().to_string(), serde_json::json!(value));
  }

  fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
    self
      .0
      .insert(field.name().to_string(), serde_json::json!(value));
  }

  fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
    self
      .0
      .insert(field.name().to_string(), serde_json::json!(value));
  }

  fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
    self
      .0
      .insert(field.name().to_string(), serde_json::json!(value));
  }

  fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
    self
      .0
      .insert(field.name().to_string(), serde_json::json!(value));
  }

  fn record_error(
    &mut self,
    field: &tracing::field::Field,
    value: &(dyn std::error::Error + 'static),
  ) {
    self.0.insert(
      field.name().to_string(),
      serde_json::json!(value.to_string()),
    );
  }

  fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
    println!("  field={} value={:?}", field.name(), value)
  }
}

pub struct CustomLayer {
  window: Window,
}

impl<S> Layer<S> for CustomLayer
where
  S: tracing::Subscriber,
{
  fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
    let mut fields = BTreeMap::new();
    let mut visitor = JsonVisitor(&mut fields);
    event.record(&mut visitor);

    let handle = thread::current();
    let name = match handle.name() {
      Some(e) => e,
      None => "",
    };

    let output = serde_json::json!({
        "thread" : name,
        "target": event.metadata().target(),
        "model":event.metadata().module_path(),
        "level": event.metadata().level().as_str(),
        "fields": fields,
    });
    self.window.emit_all("frontend_log", &output).unwrap();
  }
}

///初始化app文件夹
pub fn init_app_dir() -> bool {
  //lazy 的 处理是在第一次读取的时候 所以这里不能去读取app的值 不然会出现问题 为了让代码更加 简洁 而且在逻辑上更方便一点 我这里选择了 自己去获取一遍 为什么不能在 new方法调用init的原因也是因为会一直相互调用
  let cfg = tauri::Config::default();
  let app_path = match tauri::api::path::app_dir(&cfg) {
    None => PathBuf::new(),
    Some(p) => p.join("smovbook"),
  };
  if !&app_path.exists() {
    if let Err(_) = create_dir_all(&app_path) {
      return false;
    }
  }

  let conf = app_path.join("conf.toml");
  if !conf.exists() {
    if let Ok(_) = File::create(&conf) {
      //写入一个数据
      let a = Conf {
        tidy_folder: PathBuf::new(),
        thread: 1,
      };
      let c = toml::to_string(&a).unwrap();
      write(&conf, c).unwrap();
      return true;
    } else {
      return false;
    }
  } else {
    //如果存在 就 读取数据查看是否有不存在的项
    let mut file = match File::open(&conf) {
      Ok(f) => f,
      Err(e) => panic!("no such file {} exception:{}", conf.to_str().unwrap(), e),
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
      Ok(s) => s,
      Err(e) => panic!("Error Reading file: {}", e),
    };

    let mut confs: HashMap<String, Value> = toml::from_str(&str_val).unwrap();
    let mut flag = false;

    if confs.get("tidy_folder").eq(&None) {
      confs.insert("tidy_folder".to_string(), Value::String("".to_string()));
      flag = true;
    }

    if confs.get("thread").eq(&None) {
      confs.insert("thread".to_string(), Value::Integer(0));
      flag = true;
    }

    if flag {
      //还是需要处理这个文件被占用的可能性的
      let confs = toml::to_string(&confs).unwrap();
      // app.msg =confs;
      write(conf, &confs).unwrap();
    };
    return true;
  }
}

pub fn init_app_log(app: &mut tauri::App<Wry>) -> bool {
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

  let stdout_log = tracing_subscriber::fmt::layer()
    .with_thread_names(true)
    .with_target(false)
    .with_file(false)
    .with_line_number(false)
    .pretty(); //美化 虽然我觉的也没美化多少

  let debug_log = tracing_subscriber::fmt::layer()
    .with_writer(Arc::new(file))
    .with_ansi(false)
    .with_file(true)
    .with_filter(filter::LevelFilter::INFO);

  let now_log = stdout_log
    .with_filter(filter::LevelFilter::DEBUG) //这里的意思是 将所有info级别以上的 以stdout_log这个东西输出
    .and_then(debug_log)
    .with_filter(filter::filter_fn(|metadata| {
      //对debug_log 进行自定义过滤 debug_log为写入文件的 所以这里我只要加上 过滤条件 某个以上就好了 nice！
      !metadata.target().starts_with("frontend_log") //不存在的
    }));

  let cus = CustomLayer {
    window: match app.get_window("main") {
      Some(e) => e,
      None => todo!(),
    },
  };

  tracing_subscriber::registry()
    .with(now_log)
    .with(cus.with_filter(filter::filter_fn(|metadata| {
      //对debug_log 进行自定义过滤 debug_log为写入文件的 所以这里我只要加上 过滤条件 某个以上就好了 nice！ || metadata.level().eq(&tracing::Level::WARN)
      metadata.target().starts_with("frontend_log") || metadata.level().eq(&tracing::Level::ERROR)
      //存在的
    })))
    .init();

  info!(message = "日志系统成功载入");
  true
}

/// app配置map
pub struct App {
  pub app_dir: PathBuf,
  pub conf: Conf,
  pub msg: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct HfsConfig {
  pub config: HfsConfigs,
  pub runing: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Conf {
  pub tidy_folder: PathBuf,
  pub thread: i64, //检索线程数
}

#[derive(Deserialize, Serialize, Clone)]
pub struct HfsConfigs {
  pub address: IpAddr,

  pub port: u16,

  pub temp_dir: PathBuf,
}

impl HfsConfigs {
  pub fn default() -> HfsConfigs {
    let cfg = tauri::Config::default();
    let app_path = match tauri::api::path::app_dir(&cfg) {
      None => PathBuf::new(),
      Some(p) => p.join("SmovBook"),
    };
    HfsConfigs {
      address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
      port: 3225,
      temp_dir: app_path.join("hfs_temp"),
    }
  }
}

impl App {
  pub fn new() -> App {
    let cfg = tauri::Config::default();
    let mut app = App {
      app_dir: PathBuf::new(),
      conf: Conf {
        tidy_folder: PathBuf::new(),
        thread: 0,
      },
      msg: String::from(""),
    };
    match tauri::api::path::app_dir(&cfg) {
      None => app.app_dir = PathBuf::new(),
      Some(p) => app.app_dir = p.join("SmovBook"),
    };

    //此时文件可能不存在 调用一次app new 在懒加载不能做这个处理，，
    let conf = app.app_dir.join("conf.toml").clone();
    // if !conf.exists(){
    //   App::new();
    // }
    let mut file = match File::open(&conf) {
      Ok(f) => f,
      Err(e) => panic!("no such file {} exception:{}", conf.to_str().unwrap(), e),
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
      Ok(s) => s,
      Err(e) => panic!("Error Reading file: {}", e),
    };

    let config = toml::from_str(&str_val).unwrap();

    app.conf = config;

    app
  }
}

impl HfsConfig {
  pub fn new() -> HfsConfig {
    let cfg = tauri::Config::default();
    let app_path = match tauri::api::path::app_dir(&cfg) {
      None => PathBuf::new(),
      Some(p) => p.join("SmovBook"),
    };
    let conf = app_path.join("hfs.toml");
    let mut str_val = String::new();

    File::open(conf)
      .unwrap()
      .read_to_string(&mut str_val)
      .unwrap();

    let config: Value = toml::from_str(&str_val).unwrap();

    let config = config.as_table().unwrap().get("default").unwrap();

    let config: HfsConfigs = config.clone().try_into().unwrap();

    let hfs_config = HfsConfig {
      config,
      runing: false,
    };

    hfs_config
  }
}
/// 创建任务栏图标
#[cfg(target_os = "windows")]
pub fn create_try() -> SystemTray {
  let quit = CustomMenuItem::new("quit".to_string(), "退出");
  let set = CustomMenuItem::new("set".to_string(), "设置");
  let seek = CustomMenuItem::new("seek".to_string(), "检索列表");
  let tray_menu = SystemTrayMenu::new()
    .add_item(set)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit)
    .add_item(seek);
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
      "seek" => {
        app.get_window("seek").unwrap().unminimize().unwrap();
        app.get_window("seek").unwrap().show().unwrap();
      }
      _ => {}
    },
    SystemTrayEvent::LeftClick { .. } => {
      if let Some(window) = app.get_window("main") {
        window.show().unwrap();
        window.unminimize().unwrap();
        window.set_focus().unwrap();
        info!("handle_system_tray_event at here?");
      }
    }
    _ => {}
  }
}

/// 监听app事件
pub fn handle_app_event(app_handle: &AppHandle<Wry>, event: RunEvent) {
  match event {
    RunEvent::CloseRequested { label, api, .. } => {
      if label == "main" || label == "seek" {
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

///初始化界面阴影
#[cfg(any(target_os = "windows", target_os = "macos"))]
#[inline]
pub fn init_app_shadows(app: &mut tauri::App<Wry>) {
  use window_shadows::set_shadow;
  match app.get_window("main") {
    Some(window) => {
      set_shadow(&window, true).unwrap();
    }
    None => {}
  };
}

/// 发送拉起请求  拉起请求有问题 要修改 主要是 获取焦点的问题
fn send_wake_up() {
  let _ = thread::Builder::new()
    .name(String::from("send_wake_up"))
    .spawn(move || {
      let _s = tauri::async_runtime::block_on(async move {
        let res = UdpSocket::bind("127.0.0.1:24253").await.unwrap();
        let mut data = [0u8; 16];
        for i in 0..16 {
          data[i] = 1 as u8
        }
        res.send_to(&data, "127.0.0.1:24254").await.unwrap();
      });
    })
    .unwrap()
    .join();
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
pub fn webview2_is_installed(app: &mut tauri::App<Wry>) {
  if let Err(_) = open_reg_key() {
    unsafe {
      MessageBoxW(
        None,
        "WebView2运行时未找到，点击确定按钮去安装吧！",
        "出错啦！",
        MB_OK,
      );
      let _ = tauri::api::shell::open(
        &app.shell_scope(),
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
      let socket = UdpSocket::bind("127.0.0.1:24254").await?;
      loop {
        let mut buf = [0; 32];
        let (size, _) = socket.recv_from(&mut buf).await.expect("出现错误");
        if size == 16 {
          // check status
          let mut status = true;
          for item in &buf[0..size] {
            if *item as i32 != 1 {
              status = false;
              break;
            }
          }
          if status {
            let _ = window.emit_all("main_single", "");
          };
        };
      }
    });
}

///开发环境
//#[cfg(debug_assertions)] #[cfg(not(debug_assertions))] 环境配置
pub fn init_hfs() -> bool {
  let cfg = tauri::Config::default();
  let app_path = match tauri::api::path::app_dir(&cfg) {
    None => PathBuf::new(),
    Some(p) => p.join("SmovBook"),
  };
  let conf = app_path.join("hfs.toml");

  if !conf.exists() {
    if let Ok(_) = File::create(&conf) {
      let config = HfsConfigs::default();
      let temp_path = app_path.join("hfs_temp");
      if !temp_path.exists() {
        create_dir_all(temp_path).expect("创建hfs缓存文件夹错误");
      }

      let config = toml::Value::try_from(&config).unwrap();

      let mut map = toml::map::Map::new();

      map.insert("default".to_string(), config);

      let config = toml::to_string(&map).unwrap();

      write(&conf, config).unwrap();
    }
  }
  true
}
