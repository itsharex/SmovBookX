use core::fmt;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::thread;

use parking_lot::MutexGuard;
use rocket::error::ErrorKind;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Format, Toml};
use rocket::figment::Figment;
use rocket::fs::{FileServer, NamedFile};
use rocket::http::{ContentType, Status};
use rocket::response::content::{RawHtml, RawJson};
use rocket::response::{self, status};
use rocket::yansi::Paint;
use rocket::{Build, Error, Request, Rocket, Shutdown};
use tauri::{command, Manager, Window};
use tokio::fs::File;

use crate::model::smov::Smov;
use crate::response::response::Response;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: i8) -> String {
  format!("Hello, {} year old named {}!", age, name)
}

#[get("/<code>")]
fn forced_error(code: u16) -> Status {
  Status::new(code)
}

#[catch(404)]
fn general_not_found() -> RawHtml<&'static str> {
  RawHtml(
    r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#,
  )
}

#[catch(404)]
fn hello_not_found(req: &Request<'_>) -> RawHtml<String> {
  RawHtml(format!(
    "\
        <p>Sorry, but '{}' is not a valid path!</p>\
        <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
    req.uri()
  ))
}

#[catch(default)]
fn sergio_error() -> &'static str {
  "I...don't know what to say."
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request<'_>) -> status::Custom<String> {
  let msg = format!("{} ({})", status, req.uri());
  status::Custom(status, msg)
}

#[allow(dead_code)]
#[get("/unmanaged")]
fn unmanaged(_u8: &rocket::State<u8>, _string: &rocket::State<String>) {}

#[get("/stop")]
pub async fn stop(shutdown: Shutdown) {
  shutdown.notify()
}

#[get("/data")]
pub async fn data() -> RawJson<String> {
  let data = match Smov::get_all_smov() {
    Ok(res) => Response::new(200, Some(res), "success"),
    Err(err) => Response::new(300, None, format!("{}", err).as_str()),
  };

  let data = serde_json::to_string(&data).unwrap();

  RawJson(data)
}

struct SmovVideo(PathBuf, File, u64);

impl SmovVideo {
  pub async fn new(path: PathBuf) -> SmovVideo {
    let path_tidy = &crate::app::APP.lock().conf.tidy_folder.clone();
    let path = path_tidy.join(path);
    let file = File::open(&path).await.unwrap();
    let length = file.metadata().await.unwrap().len();
    SmovVideo(path, file, length)
  }
}

impl<'r> rocket::response::Responder<'r, 'static> for SmovVideo {
  fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
    let mut response = rocket::Response::build();
    let response = response.streamed_body(self.1);
    let range = req
      .headers()
      .get_one("Range")
      .unwrap_or_else(|| "0")
      .to_string()
      .replace("bytes=", "")
      .replace("-", "")
      .parse::<u64>()
      .unwrap();
    if let Some(ext) = self.0.extension() {
      if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
        response.raw_header("content-type", format!("{}", ct));
      }
      if ext == "mp4" {
        let content_length = format!("{}", self.2 - range);
        let content_range = format!("bytes {}-{}/{}", range, self.2 - 1, self.2);
        response
          .raw_header("cache-control", "max-age=86400")
          .raw_header("accept-ranges", "bytes")
          .raw_header("content-length", content_length)
          .raw_header("content-range", content_range)
          .status(Status { code: (206) });
          
      }
    }

    response.ok()
  }
}

struct SmovVideoFile(PathBuf, String, u64, u32, NamedFile);

impl SmovVideoFile {
  pub async fn new(path: PathBuf) -> SmovVideoFile {
    let path_buf = std::path::PathBuf::new().join(&path);
    let file = File::open(&path_buf)
      .await
      .unwrap()
      .metadata()
      .await
      .unwrap();
    let named_file = NamedFile::open(path).await.unwrap();

    let extension = path_buf
      .extension()
      .unwrap_or_else(|| &OsStr::new(""))
      .to_str()
      .unwrap_or_else(|| "")
      .to_string();
    SmovVideoFile(path_buf, extension, file.len(), 0, named_file)
  }
}

impl<'r> rocket::response::Responder<'r, 'static> for SmovVideoFile {
  fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
    let mut res = rocket::Response::build_from(self.4.respond_to(req)?);
    let range = req
      .headers()
      .get_one("Range")
      .unwrap_or_else(|| "0")
      .to_string()
      .replace("bytes=", "")
      .replace("-", "")
      .parse::<u64>()
      .unwrap();

    println!("{}", range);

    if self.1 == "mp4".to_string() {
      let content_length = format!("{}", self.2 - range);
      let content_range = format!("bytes {}-{}/{}", range, self.2 - 1, self.2);
      let _ = &res
        .raw_header("cache-control", "max-age=86400")
        .raw_header("accept-ranges", "bytes")
        .raw_header("content-length", content_length)
        .raw_header("content-range", content_range)
        .status(Status { code: (206) });
    };

    res.ok()
  }
}

#[get("/resources/<file..>")]
async fn files(file: PathBuf) -> Option<SmovVideoFile> {
  let path = &crate::app::APP.lock().conf.tidy_folder.clone();
  let path = PathBuf::new().join(path).join(file);
  Some(SmovVideoFile::new(path).await)
}

#[get("/video/<file..>")]
async fn smov_video(file: PathBuf) -> Option<SmovVideo> {
  let path = &crate::app::APP.lock().conf.tidy_folder.clone();
  let path = PathBuf::new().join(path).join(file);
  Some(SmovVideo::new(path).await)
}

fn rocket() -> Rocket<Build> {
  let figment = Figment::from(rocket::Config::default()) //由默认配置生成
    .merge(Toml::file(&crate::app::APP.lock().app_dir.join("hfs.toml")).nested()); //由toml自动生成

  let tidy_folder = &crate::app::APP.lock().conf.tidy_folder;

  rocket::custom(figment)
    .mount("/", routes![hello, forced_error])
    .register("/", catchers![general_not_found, default_catcher])
    .register("/hello", catchers![hello_not_found])
    .register("/hello/Sergio", catchers![sergio_error])
    .mount("/", routes![stop])
    .mount("/", routes![data])
    .mount("/", routes![files])
    .mount("/", routes![smov_video])
    .mount("/SmovStatic", FileServer::from(tidy_folder))
}

#[command]
pub async fn rocket_main(window: Window) {
  //需要一个服务器是否正在运行的状态 需要随时能够停止或重启服务器 不需要服务器访问权限 需要错误返回原因
  let runing = crate::app::HFSCONFIG.lock().clone().runing;
  let windows_th = window.get_window("main").unwrap();
  if !runing {
    let handle = thread::Builder::new()
      .name(String::from("hfs"))
      .spawn(move || {
        let _s = tauri::async_runtime::block_on(async move {
          if let Err(e) = rocket()
            .attach(AdHoc::on_liftoff("Liftoff Printer", |_| {
              Box::pin(async move {
                let mut config = crate::app::HFSCONFIG.lock();
                config.runing = true;
                MutexGuard::unlock_fair(config);
                windows_th
                  .emit(
                    "HFS://OperatingStatus",
                    Response::ok(Some(true), "hfs服务器开启"),
                  )
                  .unwrap();
              })
            }))
            .launch()
            .await
          {
            let mut config = crate::app::HFSCONFIG.lock();
            config.runing = false;
            MutexGuard::unlock_fair(config);
            drop(e);
          }
        });
      })
      .unwrap();

    let _handle = match handle.join() {
      Ok(_) => {
        let _ = &window
          .emit(
            "HFS://OperatingStatus",
            Response::ok(Some(false), "hfs服务器正常关闭"),
          )
          .unwrap();
        return;
      }
      Err(err) => {
        let msg = err.downcast::<String>().expect(""); //得到 panic 中的错误 需要控制类型 str 或 string
        let _ = &window
          .emit(
            "HFS://OperatingStatus",
            Response::err(Some(false), format!("hfs服务器非正常关闭:{}", msg).as_str()),
          )
          .unwrap();
        return;
      }
    };
  } else {
    window
      .emit(
        "HFS://OperatingStatus",
        Response::ok(Some(true), "hfs服务器开启"),
      )
      .unwrap();
  }
}

#[command]
pub async fn request_shutdown(window: Window) {
  std::thread::sleep(std::time::Duration::from_millis(200));
  let hfs = &crate::app::HFSCONFIG.lock().clone();
  if hfs.runing {
    let port = &crate::app::HFSCONFIG.lock().clone().config.port;
    let _ = reqwest::get(format!("http://127.0.0.1:{}/stop", port)).await;
    let config = &mut crate::app::HFSCONFIG.lock();
    config.runing = false;
    return;
  } else {
    window
      .emit(
        "HFS://OperatingStatus",
        Response::ok(Some(false), "hfs服务器正常关闭"),
      )
      .unwrap();
  }
}

#[command]
pub async fn hfs_is_runing() -> Response<Option<bool>> {
  let hfs = &crate::app::HFSCONFIG.lock().clone().runing;
  Response::new(200, Some(hfs.clone()), "success")
}

fn drop(error: Error) {
  if std::thread::panicking() {
    return;
  }

  match error.kind() {
    ErrorKind::Bind(ref e) => {
      error!("端口已被占用"); //Rocket failed to bind network socket to given address/port.
                              // info_!("{}", e);
      panic!("{}", format!("{}", e).as_str());
    }
    ErrorKind::Io(ref e) => {
      error!("Rocket failed to launch due to an I/O error.");
      info_!("{}", e);
      panic!("aborting due to i/o error");
    }
    ErrorKind::Collisions(ref collisions) => {
      fn log_collisions<T: fmt::Display>(kind: &str, collisions: &[(T, T)]) {
        if collisions.is_empty() {
          return;
        }

        error!(
          "Rocket failed to launch due to the following {} collisions:",
          kind
        );
        for &(ref a, ref b) in collisions {
          info_!("{} {} {}", a, Paint::red("collides with").italic(), b)
        }
      }

      log_collisions("route", &collisions.routes);
      log_collisions("catcher", &collisions.catchers);

      info_!("Note: Route collisions can usually be resolved by ranking routes.");
      panic!("routing collisions detected");
    }
    ErrorKind::FailedFairings(ref failures) => {
      error!("Rocket failed to launch due to failing fairings:");
      for fairing in failures {
        info_!("{}", fairing.name);
      }

      panic!("aborting due to fairing failure(s)");
    }
    ErrorKind::InsecureSecretKey(profile) => {
      error!("secrets enabled in non-debug without `secret_key`");
      info_!("selected profile: {}", Paint::default(profile).bold());
      info_!("disable `secrets` feature or configure a `secret_key`");
      panic!("aborting due to insecure configuration")
    }
    ErrorKind::Config(error) => {
      rocket::config::pretty_print_error(error.clone());
      panic!("aborting due to invalid configuration")
    }
    ErrorKind::SentinelAborts(ref failures) => {
      error!("Rocket failed to launch due to aborting sentinels:");
      for sentry in failures {
        let name = Paint::default(sentry.type_name).bold();
        let (file, line, col) = sentry.location;
        info_!("{} ({}:{}:{})", name, file, line, col);
      }

      panic!("aborting due to sentinel-triggered abort(s)");
    }
    _ => {}
  }
}
