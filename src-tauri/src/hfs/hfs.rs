use core::fmt;
use std::thread;

use rocket::error::ErrorKind;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::yansi::Paint;
use rocket::{Build, Error, Request, Rocket, Shutdown};
use tauri::command;

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

fn rocket() -> Rocket<Build> {
  rocket::build()
    .mount("/", routes![hello, forced_error])
    .register("/", catchers![general_not_found, default_catcher])
    .register("/hello", catchers![hello_not_found])
    .register("/hello/Sergio", catchers![sergio_error])
    .mount("/", routes![stop])
    
}

#[command]
pub async fn rocket_main() -> Response<Option<i32>> {
  //需要一个服务器是否正在运行的状态 需要随时能够停止或重启服务器 不需要服务器访问权限 需要错误返回原因

  let handle = thread::Builder::new()
    .name(String::from("hfs"))
    .spawn(move || {
      let _s = tauri::async_runtime::block_on(async move {
        if let Err(e) = rocket().launch().await {
          drop(e);
        };
      }); 
    })
    .unwrap();

  let _handle = match handle.join() {
    Ok(_) => return Response::new(200, Some(1), "success"),
    Err(err) => {
      let s = err.downcast::<String>().expect("");
      return Response::err(None, &s);
    }
  };
}

async fn request_shutdown() {
    std::thread::sleep(std::time::Duration::from_millis(200));
    let _ = reqwest::get(format!("http://127.0.0.1:{}/stop", 4000)).await;
}

#[command]
pub async fn hfs_is_runing() -> Response<Option<bool>> {
  Response::new(200, Some(true), "success")
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
