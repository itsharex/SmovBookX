use std::path::PathBuf;

use crate::{
  crawler::{error::CrawlerErr, save_pic::sava_pic_sync},
  model::smov::{RetrievingSmov, SmovSeek},
  serve::file::TidySmov,
  util::smov_format::SmovName,
};
use anyhow::Result;

///考虑在爬虫实现热更新 模板引擎的错误尝试 模板引擎可能并不适合爬虫 有没有直接从云端获取结构的办法 哎 到时候再解决吧
use scraper::{Html, Selector};
use tauri::command;
use tokio::task::block_in_place;

use super::client::{CLIENT, MAIN_URL};

#[command]
pub async fn smov_crawler(retrieving_smov: RetrievingSmov) {
  // let s = tauri::async_runtime::spawn(smov_crawler_program(format, id)).await;
  let format = SmovName::format_smov_name(&retrieving_smov.seek_name);
  match smov_crawler_program(format, retrieving_smov.id).await {
    Ok(res) => println!("{}", res),
    Err(err) => println!("{}", err.to_string()),
  }; //好像不等待就好了 不加await 就不会执行了 为啥呢
}

pub async fn smov_crawler_program(format: String, id: i64) -> Result<bool> {
  let url = format!("{}/search?q={}&f=all", *MAIN_URL, format);

  let selector = Selector::parse(".movie-list").unwrap();

  let fragment = match get_temp_sync(&url) {
    Ok(html) => html,
    Err(err) => return Err(err),
  };

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

  let title = item_path
    .value()
    .attr("title")
    .unwrap_or_else(|| "")
    .to_string();

  let item_url = item_path.value().attr("href").unwrap_or_else(|| "");

  let thumbs_url = img.value().attr("src").unwrap_or_else(|| "");

  println!("{}", title);

  match sava_pic_sync(
    thumbs_url.to_string(),
    format!("thumbs_{}.jpg", &name),
    PathBuf::from(&img_to_path),
  ) {
    Err(err) => return Err(err),
    _ => {}
  };

  let url = format!("{}{}", *MAIN_URL, item_url.to_string());

  println!("{}", url);

  let fragment = match get_temp_sync(&url) {
    Ok(html) => html,
    Err(err) => return Err(err),
  };

  let mut smov_seek = SmovSeek {
    id,
    name: String::from(&name),
    title,
    format,
    release_time: String::new(),
    duration: 0,
    publishers: String::new(),
    makers: String::new(),
    series: String::from("无系列"),
    directors: String::new(),
    tags: Vec::new(),
    actors: Vec::new(),
  };

  let selector = Selector::parse(".video-meta-panel").unwrap();

  let mut video_meta_panel = fragment.select(&selector);

  let video_meta_panel = match video_meta_panel.next() {
    Some(el) => el,
    None => {
      return Err(anyhow::Error::new(CrawlerErr::ItemNotFound));
    }
  };

  let selector = Selector::parse("img").unwrap();

  let main_img = video_meta_panel
    .select(&selector)
    .next()
    .unwrap()
    .value()
    .attr("src")
    .unwrap_or_else(|| "");

  match sava_pic_sync(
    thumbs_url.to_string(),
    format!("main_{}.jpg", &name),
    PathBuf::from(&img_to_path),
  ) {
    Err(err) => return Err(err),
    _ => {}
  };

  let selector = Selector::parse(".panel-block").unwrap();

  let strong_selector = Selector::parse("strong").unwrap();

  for detail in video_meta_panel.select(&selector) {
    if let Some(strong_type_el) = detail.select(&strong_selector).next(){
         match strong_type_el.inner_html().as_str() {
          " 時長:" =>{},
          _=>{}
             
         }
    };
     
  }

  return Ok(true);
}

async fn _get_temp(url: String) -> String {
  let res = CLIENT.get(url).send().await.expect("访问出现错误");

  res.text().await.expect("无法格式化")
}

//改造 返回错误 而不是返回Html
fn get_temp_sync(url: &String) -> Result<Html> {
  let url1 = String::from(url);
  let url2 = String::from(url);
  block_in_place(|| match reqwest::blocking::get(url1) {
    Ok(res) => Ok(Html::parse_fragment(
      &res.text().expect("字符串没提取出来懒得处理的错误"),
    )),
    Err(err) => {
      return Err(anyhow::Error::new(CrawlerErr::NetworkError {
        url: url2,
        msg: err.to_string(),
      }));
    }
  })
}
