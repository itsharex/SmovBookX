use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::crawler::client::CLIENT;

pub async fn save_pic(url: String, name: String, path: PathBuf) -> Result<()> {
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