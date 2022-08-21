use crate::{
  hfs::{api::system_api, handle::handler_404},
  response::response::Response,
};
use axum::{handler::Handler, Router};
use parking_lot::MutexGuard;
use std::net::SocketAddr;
use tauri::{command, Window};
use tokio::{signal, sync::mpsc};
use tower_http::trace::TraceLayer;

///当前如果出现运行时错误 是出不来的 虽然出现的可能性不大 但是还是需要做的
#[command]
pub async fn run_hfs(window: Window) {
  let conf = &mut crate::app::HFSCONFIG.lock().clone();

  if conf.runing {
    window
      .emit(
        "HFS://OperatingStatus",
        Response::ok(Some(true), "hfs服务器开启"),
      )
      .unwrap();
  } else {
    let app: _ = Router::new()
      .nest("/smovbook", system_api())
      .fallback(handler_404.into_service())
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
        let msg = format!("hfs服务器运行出现错误{}", err);
        window
          .emit("HFS://OperatingStatus", Response::err(Some(false), &msg))
          .unwrap();
      }
      _ => {}
    };
  }
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
