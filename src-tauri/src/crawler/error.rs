use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlerErr {
  #[error("未爬取到数据")]
  NotFound,
  #[error("未爬取到明细数据")]
  ItemNotFound,
  #[error("访问出现错误:{msg},URL:{url}")]
  NetworkError { url: String, msg: String },
  #[error("IO错误:{msg},path:{path}")]
  IOError { msg: String, path: String },
  #[error("其他错误")]
  #[warn(dead_code)]
  OtherError(String),
  #[error("unknown error")]
  Unknown,
}

impl CrawlerErr {}
