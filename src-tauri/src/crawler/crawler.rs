use std::path::PathBuf;

use anyhow::Result;
///考虑在爬虫实现热更新 模板引擎的错误尝试 模板引擎可能并不适合爬虫 有没有直接从云端获取结构的办法 哎 到时候再解决吧
use reqwest::header::HeaderMap;
use scraper::{Html, Selector};
use tauri::command;
use thiserror::Error;

use crate::{crawler::save_pic::sava_pic, serve::file::TidySmov, util::smov_format::SmovName};

lazy_static! {
  static ref HEADER: HeaderMap = {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".parse().unwrap());
    headers.insert("cookie", "theme=auto; locale=zh; _ym_uid=1643076534203711663; _ym_d=1643076534; _ym_isad=1; over18=1; _jdb_session=u9TcLXlGGbtvm9gGwZYEpinDW9hp8wUpxrV1z88%2Bu6v7DTLIvBn9rUCQBt7O33JtmzPizK4a67uE8E75PJ56YhJQaocudrRi%2B4Ly025mTYamqzR%2FLDSfG5E%2FI32MC05KRngYkB04O%2Blli1jEvGzLLjH7GMDjERWejUQqwWtYVKEOhf2tfP7%2FPk%2BFo8Rg86S1Tai7Zg7Gc1rB0JwUqIMETFc%2BIToWoZ0jNTXWliRGSlhXpvO4Akn%2FuaBu771kG1uiSK0gQPCDTG9hheuFAjjfI0p%2FFV4b4usCkPiZZH3I2vWCM7S%2B4u6uk%2BXs--YVqvN%2Byh43AE6xyR--J5NZMl5Ko12LNJRzk%2Fzbpw%3D%3D".parse().unwrap());
    headers.insert(
      "jdsignature",
      "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
        .parse()
        .unwrap(),
    );
    headers
  };
  static ref MAIN_URL: String = String::from("https://javdb36.com");
}

#[command]
pub async fn smov_crawler(format: String, id: i64) {
  // let s = tauri::async_runtime::spawn(smov_crawler_program(format, id)).await;
  smov_crawler_program(format, id).await;
}

pub async fn smov_crawler_program(format: String, id: i64) -> Result<bool> {
  let header = HeaderMap::clone(&*HEADER);

  let url = format!("{}/search?q={}&f=all", *MAIN_URL, format);

  let client = reqwest::Client::new();

  let res = client
    .get(url)
    .headers(header.clone())
    .send()
    .await
    .expect("访问出现错误");

  let text = res.text().await.expect("无法格式化");

  let fragment = Html::parse_fragment(&text);

  let selector = Selector::parse(".movie-list").unwrap();

  let movie_list = match fragment.select(&selector).next() {
    Some(list) => list,
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let movie_list_item_select = Selector::parse(".item").unwrap();

  let name_selector = Selector::parse("strong").unwrap();

  let smov_item = movie_list
    .select(&movie_list_item_select)
    .filter(|&movie_item| {
      if let Some(name) = movie_item.select(&name_selector).next() {
        SmovName::format_smov_name(&name.inner_html()).eq(&format)
      } else {
        false
      }
    })
    .collect::<Vec<_>>();

  let smov_item = match smov_item.first() {
    Some(item) => item,
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let name = match smov_item.select(&name_selector).next() {
    Some(name) => name.inner_html(),
    None => return Err(anyhow::Error::new(CrawlerErr::NotFound)),
  };

  let s = TidySmov {
    id: &id,
    name: &name,
  };

  let img_to_path = match s.tidy() {
    Ok(n) => n,
    Err(err) => return Err(err),
  };

  let item_path = smov_item
    .select(&Selector::parse("a").unwrap())
    .next()
    .unwrap();
  let img = smov_item
    .select(&Selector::parse("img").unwrap())
    .next()
    .unwrap();

  let title = item_path.value().attr("title").unwrap_or_else(|| "");

  let item_url = item_path.value().attr("href").unwrap_or_else(|| "");

  let thumbs_url = img.value().attr("src").unwrap_or_else(|| "");

  // sava_pic(
  //   String::from(thumbs_url),
  //   format!("thumbs_{}.jpg", name),
  //   PathBuf::from(&img_to_path),
  // )
  // .await; //异步拉取数据会造成错误被跳过 同步拉取会出现错误我tm 因为一直出现错误 更新tauri版本 烦死了

  // //异步中不能继续创建异步代码 所以如果需要新建一个异步程序 需要spawn
  // let s = tokio::spawn({
  //   let url = String::from(thumbs_url);
  //   let name = format!("thumbs_{}.jpg", name);
  //   let path = PathBuf::from(&img_to_path);
  //   sava_pic(url, name, path)
  // });

  return Ok(true);
}

#[derive(Error, Debug)]
pub enum CrawlerErr {
  #[error("未爬取到数据")]
  NotFound,
}
