use anyhow::Result;
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

///保存图片 设置错误重试
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

  block_in_place(
    || match reqwest::blocking::get(&url) {
      Ok(mut res) => match res.copy_to(&mut file) {
        Ok(_) => Ok(()),
        Err(err) => {
          return Err(anyhow::Error::new(CrawlerErr::IOError {
            msg: err.to_string(),
            path: path_str,
          }))
        }
      },
      Err(err) => {
        return Err(anyhow::Error::new(CrawlerErr::NetworkError {
          url: String::from(&url),
          msg: err.to_string(),
        }))
      }
    }, //reqwest::blocking::get(url).expect("请求失败").copy_to(&mut file).expect("")
  )
}
