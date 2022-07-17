use anyhow::Result;
use scraper::Html;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use tokio::task::block_in_place;

use crate::crawler::client::CLIENT;

use super::error::CrawlerErr;

pub async fn _save_pic(url: String, name: String, path: PathBuf) -> Result<()> {
  let pic_path = path.join(name);

  println!("执行,{},{:?}", url, pic_path);

  let msg = format!(
    "保存图片url:{} => path:{}",
    url,
    path.as_os_str().to_str().unwrap_or_else(|| "none")
  );

  tracing::info!(target: "frontend_log",message = msg.as_str());

  let res = CLIENT.get(url).send().await?;
  let res = res.bytes().await?;

  let mut file = match File::create(&pic_path) {
    Err(why) => panic!("couldn't create {}", why),
    Ok(file) => file,
  };

  let content = res.bytes();
  let data: std::result::Result<Vec<_>, _> = content.collect();
  file.write_all(&data.unwrap()).unwrap();

  Ok(())
}

pub fn sava_pic_sync(url: String, name: String, path: PathBuf) -> Result<()> {
  let pic_path = path.join(name);
  let path_str = String::from(&pic_path.to_str().unwrap_or_else(|| "").to_string());
  let mut file = match File::create(&pic_path) {
    Ok(file) => file,
    Err(err) => {
      return Err(anyhow::Error::new(CrawlerErr::IOError {
        msg: err.to_string(),
        path: path_str,
      }))
    }
  };

  //重试机制
  //错误重试三次
  tracing::info!("保存图片{}至{}", &url, String::from(&path_str));
  block_in_place(|| {
    let mut ret: Result<(), anyhow::Error> = Err(anyhow::Error::new(CrawlerErr::Unknown));
    let mut index = 1;

    while index != 4 {
      match reqwest::blocking::get(&url) {
        Ok(mut res) => match res.copy_to(&mut file) {
          Ok(_) => return Ok(()),
          Err(err) => {
            return Err(anyhow::Error::new(CrawlerErr::IOError {
              msg: err.to_string(),
              path: String::from(&path_str),
            }));
          }
        },
        Err(err) => {
          tracing::error!(
            "保存图片{}出现错误，重新提取，错误原因:{}",
            String::from(&path_str),
            err.to_string()
          );
          ret = Err(anyhow::Error::new(CrawlerErr::NetworkError {
            url: String::from(&url),
            msg: err.to_string(),
          }));
          index += 1;
        }
      };
    }
    ret
  })
}


//改造 返回错误 而不是返回Html
pub fn get_temp_sync(url: &String) -> Result<Html> {
  block_in_place(|| {
    let mut ret: Result<scraper::Html, anyhow::Error> =
      Err(anyhow::Error::new(CrawlerErr::Unknown));
    let mut index = 1;

    while index != 4 {
      match reqwest::blocking::get(String::from(url)) {
        Ok(res) => {
          let text = &res.text();
          let text = match text {
            Ok(text) => text,
            Err(_) => {
              index += 1;
              continue;
            },
          };

          return Ok(Html::parse_fragment(text));
        }
        Err(err) => {
          ret = Err(anyhow::Error::new(CrawlerErr::NetworkError {
            url: String::from(url),
            msg: err.to_string(),
          }));
          index += 1;
        }
      }
    }
    ret
  })
}
