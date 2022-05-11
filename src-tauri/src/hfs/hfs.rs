use core::fmt;
use std::path::PathBuf;
use std::thread;

use parking_lot::MutexGuard;
use rocket::error::ErrorKind;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Format, Toml};
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::stream::ByteStream;
use rocket::response::{content, status};
use rocket::yansi::Paint;
use rocket::{Build, Error, Request, Rocket, Shutdown};
use tauri::{command, Manager, Window};

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
fn general_not_found() -> content::Html<&'static str> {
  content::Html(
    r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/hello/Sergio/100">hello!</a>
    "#,
  )
}

#[catch(404)]
fn hello_not_found(req: &Request<'_>) -> content::Html<String> {
  content::Html(format!(
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

// #[get("/<file..>")]
// fn videoTest(file: PathBuf) -> ByteStream![&'static [u8]] {
//   let path = &crate::app::APP.lock().conf.tidy_folder.clone();
//   let path = path.join("ABP-408").join("ABP-408.mp4").to_str().unwrap();
//   // let file = std::fs::File::open(path).unwrap();
//   include_bytes!(path)
// }

#[get("/data")]
pub async fn data() -> content::Json<String> {
  let data = match Smov::get_all_smov() {
    Ok(res) => Response::new(200, Some(res), "success"),
    Err(err) => Response::new(300, None, format!("{}", err).as_str()),
  };

  let data = serde_json::to_string(&data).unwrap();

  content::Json(data)
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
    // .mount("/test",routes![videoTest])
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
    ErrorKind::Runtime(ref err) => {
      error!("An error occurred in the runtime:");
      info_!("{}", err);
      panic!("aborting due to runtime failure");
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
