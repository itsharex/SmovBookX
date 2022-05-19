pub async fn sava_pic(
    url: &String,
    name: &String,
    path: &PathBuf,
    client: &Client,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let pic_path = path.join(name);
  
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".parse().unwrap());
    headers.insert("cookie", "theme=auto; locale=zh; _ym_uid=1643076534203711663; _ym_d=1643076534; _ym_isad=1; over18=1; _jdb_session=u9TcLXlGGbtvm9gGwZYEpinDW9hp8wUpxrV1z88%2Bu6v7DTLIvBn9rUCQBt7O33JtmzPizK4a67uE8E75PJ56YhJQaocudrRi%2B4Ly025mTYamqzR%2FLDSfG5E%2FI32MC05KRngYkB04O%2Blli1jEvGzLLjH7GMDjERWejUQqwWtYVKEOhf2tfP7%2FPk%2BFo8Rg86S1Tai7Zg7Gc1rB0JwUqIMETFc%2BIToWoZ0jNTXWliRGSlhXpvO4Akn%2FuaBu771kG1uiSK0gQPCDTG9hheuFAjjfI0p%2FFV4b4usCkPiZZH3I2vWCM7S%2B4u6uk%2BXs--YVqvN%2Byh43AE6xyR--J5NZMl5Ko12LNJRzk%2Fzbpw%3D%3D".parse().unwrap());
    headers.insert(
      "jdsignature",
      "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
        .parse()
        .unwrap(),
    );
  
    let msg = format!(
      "保存图片url:{} => path:{}",
      url,
      path.as_os_str().to_str().unwrap_or_else(|| "none")
    );
  
    tracing::info!(target: "frontend_log",message = msg.as_str());
  
    let res = client
      .get(url)
      .headers(headers)
      .send()
      .await?
      .bytes()
      .await?;
  
    let mut file = match File::create(&pic_path) {
      Err(why) => panic!("couldn't create {}", why),
      Ok(file) => file,
    };
  
    let content = res.bytes();
    let data: std::result::Result<Vec<_>, _> = content.collect();
    file.write_all(&data.unwrap())?;
  
    Ok(())
  }