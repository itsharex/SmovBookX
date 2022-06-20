use std::path::PathBuf;

use anyhow::Result;
///考虑在爬虫实现热更新 模板引擎的错误尝试 模板引擎可能并不适合爬虫 有没有直接从云端获取结构的办法 哎 到时候再解决吧
use scraper::{Html, Selector};
use tauri::command;
use thiserror::Error;
use tokio::sync::oneshot;

use crate::{serve::file::TidySmov, util::smov_format::SmovName};

use super::{
  client::{CLIENT, MAIN_URL},
  save_pic::save_pic,
};

#[command]
pub async fn smov_crawler(format: String, id: i64) {
  // let s = tauri::async_runtime::spawn(smov_crawler_program(format, id)).await;
  match smov_crawler_program(format, id).await {
    Ok(res) => println!("{}", res),
    Err(err) => println!("{}", err.to_string()),
  }; //好像不等待就好了 不加await 就不会执行了 为啥呢
}

pub async fn smov_crawler_program(format: String, id: i64) -> Result<bool> {
  let (tx, rx) = oneshot::channel();

  let url = format!("{}/search?q={}&f=all", *MAIN_URL, format);

  let s = tx.send(CLIENT.get(url).send().await.expect("访问出现错误")).unwrap();

  // let res = CLIENT.get(url).send().await.expect("访问出现错误");

  let res = rx.await?;

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

  //异步函数可以看做被await分割成了许多个小函数，它们不一定在同一个线程里被执行，所以一般要用异步的基础设施。
  //用异步锁而不应当用Cell
  //那异步锁怎么写
  // save_pic(
  //   thumbs_url.to_string(),
  //   format!("thumbs_{}.jpg", name),
  //   PathBuf::from(&img_to_path),
  // )
  // .await;

  let s = tx.send(save_pic(
    thumbs_url.to_string(),
    format!("thumbs_{}.jpg", name),
    PathBuf::from(&img_to_path),
  )
  .await).unwrap();

  //异步拉取数据会造成错误被跳过 同步拉取会出现错误我tm 因为一直出现错误 更新tauri版本 烦死了

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
