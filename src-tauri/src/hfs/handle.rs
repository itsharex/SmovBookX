use std::io;

use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use parking_lot::lock_api::MutexGuard;
use tauri::Window;
use tokio::{signal, sync::mpsc};

pub async fn file_handle_error(_err: io::Error) -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}

pub async fn render_not_found<T>(request: axum::http::Request<T>) -> std::io::Result<Response> {
  std::io::Result::Ok(format!("{} not found", request.uri()).into_response())
}

pub async fn shutdown_signal(window: &Window) {
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
          crate::response::response::Response::ok(Some(false), "hfs服务器正常关闭"),
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
