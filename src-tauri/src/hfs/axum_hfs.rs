use crate::{
  hfs::{
    api::system_api,
    handle::{handler_404, shutdown_signal},
  },
  response::response::Response,
};
use axum::{handler::Handler, Router};
use parking_lot::MutexGuard;
use std::net::SocketAddr;
use tauri::{command, Window};

use tower_http::trace::TraceLayer;

///当前如果出现运行时错误 是出不来的 虽然出现的可能性不大 但是还是需要做的
/// https://docs.rs/axum-server/latest/axum_server/struct.Handle.html#method.graceful_shutdown
/// https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs
/// https://github.com/FiloSottile/mkcert 本地打包证书
/// https://letsencrypt.osfipin.com/ 证书申请
/// 应当提供一张默认的 没用的证书  给选择的机会
/// 尝试使用 axum_server实现 实现https的功能 
/// 不允许https 如果需要https 用Nginx 代理
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
