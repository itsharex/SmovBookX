use axum::extract::Query;

use crate::model::smov::Smov;

use super::res::{ListData, PageParams, Res};

pub async fn get_data_all() -> Res<Vec<Smov>> {
  match Smov::get_all_smov() {
    Ok(res) => Res::with_data(res),
    Err(err) => Res::with_err(&err.to_string()),
  }
}

pub async fn get_data_pagination(Query(page_params): Query<PageParams>) -> Res<ListData<Smov>> {
  match Smov::get_smov_pagination(page_params) {
    Ok(res) => Res::with_data(res),
    Err(err) => Res::with_err(&err.to_string()),
  }
}

pub async fn error_test() -> Res<String> {
  Res::with_err(&"错误测试".to_string())
}
