use crate::{model::smov::Smov, response::response::Response};
use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, get_service},
  Router, Json,
};
use parking_lot::MutexGuard;
use std::{io, net::SocketAddr, thread};
use tauri::{command, Window};
use tokio::{signal, sync::mpsc};
use tower_http::{services::ServeDir, trace::TraceLayer};

///当前如果出现运行时错误 是出不来的 虽然出现的可能性不大 但是还是需要做的
#[command]
pub async fn run_hfs(window: Window) {
  let conf = &mut crate::app::HFSCONFIG.lock().clone();

  let tidy_folder = &crate::app::APP.lock().clone().conf.tidy_folder;

  if conf.runing {
    window
      .emit(
        "HFS://OperatingStatus",
        Response::ok(Some(true), "hfs服务器开启"),
      )
      .unwrap();
  } else {
    let app: _ = Router::new()
      .route("/foo", get(|| async { "Hi from /foo" }))
      .route(
        "/resources",
        get_service(ServeDir::new(tidy_folder)).handle_error(handle_error),
      )
      .route("/data",get(get_data))
      .fallback(get_service(ServeDir::new(tidy_folder)).handle_error(handle_error))
      .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((conf.config.address, conf.config.port));
    tracing::info!("listening on {}", addr);

    let server = match axum::Server::try_bind(&addr) {
      Ok(ser) => ser,
      Err(err) => {
        let msg = format!("{}", err);
        let msg = msg.as_str();
        window
          .emit("HFS://OperatingStatus", Response::err(Some(false), &msg))
          .unwrap();
        panic!("{}", msg)
      }
    };

    let server = server
      .serve(app.into_make_service())
      .with_graceful_shutdown(shutdown_signal(&window)); //和这里捕获错误

    window
      .emit(
        "HFS://OperatingStatus",
        Response::ok(Some(true), "hfs服务器开启"),
      )
      .unwrap();

    //放入异步运行时处理
    tauri::async_runtime::spawn_blocking(move || {
      let mut config = crate::app::HFSCONFIG.lock();
      config.runing = true;
      MutexGuard::unlock_fair(config);
    })
    .await
    .unwrap();

    match server.await {
      Err(err) => {
        let msg = format!("hfs服务器运行出现从错误{}", err);
        window
          .emit("HFS://OperatingStatus", Response::err(Some(false), &msg))
          .unwrap();
      }
      _ => {}
    };
  }
}

///过时  已弃用
#[command]
pub async fn _run_hfs1(window: Window) {
  let runing = &crate::app::HFSCONFIG.lock().clone().runing;

  if *runing {
    window
      .emit(
        "HFS://OperatingStatus",
        Response::ok(Some(true), "hfs服务器开启"),
      )
      .unwrap();
  } else {
    let _handle = thread::Builder::new()
      .name(String::from("hfs"))
      .spawn(move || {
        let _s = tauri::async_runtime::block_on(async move {
          let conf = &crate::app::HFSCONFIG.lock().clone();

          let _app_conf = &crate::app::APP.lock().clone();

          let app: _ = Router::new()
            .route("/foo", get(|| async { "Hi from /foo" }))
            .fallback(
              get_service(ServeDir::new("C:\\Users\\Leri\\Videos\\ZL\\IPX-215\\"))
                .handle_error(handle_error),
            )
            .layer(TraceLayer::new_for_http());

          let addr = SocketAddr::from((conf.config.address, conf.config.port));
          tracing::debug!("listening on {}", addr);

          let server = match axum::Server::try_bind(&addr) {
            Ok(ser) => ser,
            Err(err) => {
              let msg = format!("{}", err);
              let msg = msg.as_str();
              window
                .emit("HFS://OperatingStatus", Response::err(Some(false), &msg))
                .unwrap();
              panic!("{}", msg)
            }
          };

          let server = server.serve(app.into_make_service()); //和这里捕获错误

          window
            .emit(
              "HFS://OperatingStatus",
              Response::ok(Some(true), "hfs服务器开启"),
            )
            .unwrap();

          let mut config = crate::app::HFSCONFIG.lock();
          config.runing = false;
          MutexGuard::unlock_fair(config);

          match server
            .with_graceful_shutdown(shutdown_signal(&window))
            .await
          {
            Err(err) => {
              let msg = format!("hfs服务器运行出现从错误{}", err);
              window
                .emit("HFS://OperatingStatus", Response::err(Some(false), &msg))
                .unwrap();
            }
            _ => {}
          };
        });
      });
  }
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn shutdown_signal(window: &Window) {
  let (tx, mut rx) = mpsc::unbounded_channel();

  let ctrl_c = async {
    signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };

  let _shutdown = window.once("HFS://ShutDown", move |_event| {
    if let Err(e) = tx.send(()) {
      println!("failed to send value {:?}", e);
    }
  });

  //上面是个监听 我是否能做个 延时测试是否能正常关闭

  #[cfg(unix)]
  let terminate = async {
    signal::unix::signal(signal::unix::SignalKind::terminate())
      .expect("failed to install signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  //运行 ctrl_c 或 terminate 当有一个运行就跳出的意思吧 还有一个接收到前端消息
  tokio::select! {
      _ = ctrl_c => {},
      _ = terminate => {},
      _ = rx.recv() => {
        let _ = &window
        .emit(
          "HFS://OperatingStatus",
          Response::ok(Some(false), "hfs服务器正常关闭"),
        )
        .unwrap();

        tauri::async_runtime::spawn_blocking(move || {
          let mut config = crate::app::HFSCONFIG.lock();
          config.runing = false;
          MutexGuard::unlock_fair(config);
        }).await.unwrap();
      },
  }
}

pub async fn get_data() -> Json<Response<Option<Vec<Smov>>>> {
  let data = match Smov::get_all_smov() {
    Ok(res) => Response::new(200, Some(res), "success"),
    Err(err) => Response::new(300, None, format!("{}", err).as_str()),
  };

  Json(data)
}
