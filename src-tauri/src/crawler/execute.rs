use crate::crawler::template::Temp;
use axum::http::HeaderMap;
use kuchiki::traits::TendrilSink;
use tauri::command;

lazy_static! {
  pub static ref MODEL: Temp = Temp::new();
}

#[command]
pub async fn execute() {
  let mut headers = HeaderMap::new();
  headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.63 Safari/537.36 Edg/102.0.1245.33".parse().unwrap());
  headers.insert("cookie", "list_mode=h; theme=auto; locale=zh; _ym_d=1653481970; _ym_uid=165348197013084594; over18=1; hide_app_banner=1; _jdb_session=slixKp28xM3/FDyO0XYOppTnvmU+uTJFeWRmNPZnDtQRG171nOQLzoXJgPxEYFIdT6AamvLDcZEuv79vxDW1MwgT4iuEfpSDclRyxwZzHCV0L3QTDnaE1ERsyqe4HGrw4WMS99nBe4UQ4eaTh0HwA5ooCwJyZBNsYxaIqompPKA9QeaPg0CRnMLzlMWf5hND2/Mb7z2EDww0jZiJHL/I8V4GoPBJZVvGeeHT+JRHGZn5ZRNE16vEoSScPKU9H9yOXt5juTDCnnCtVqaeIT0R8frbPHiIXihZNpgwcWAuk2ZvCyxWVAc3oW4V--OjlkDjWAFfvFb8TJ--n4DQ6vBbnhD75DiEd9VHmg==; __cf_bm=caOboU9ZjcepDriaFmXURAvZSLi6WUfm88mzr8urf7Q-1654689852-0-AVOdj3majisyZlJO5qw2RD5EKqDMQR18kBtPsn7lzfIs6ZBk1CtLrmxbuJQP4kL8qUc17BizgvDZQ41fNkn2aBJB+/SIKy6QYBEHMWxrbo1FCNc6z0I5NQMEXiapUp6OtA==; _ym_isad=1".parse().unwrap());
  headers.insert(
    "jdsignature",
    "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
      .parse()
      .unwrap(),
  );

  let crtmp = MODEL.cr_tmp.clone();

  let main_url = MODEL.url.clone();

  let format = "miaa213";

  let url = format!("{}/search?q={}&f=all", main_url, format);

  tracing::info!("{}", url);

  //获取主html
  let client = reqwest::Client::new();
  let res = client
    .get(url)
    .headers(headers.clone())
    .send()
    .await
    .expect("访问出现错误");

  let text = res.text().await.expect("无法格式化");

  let document = kuchiki::parse_html().one(text);

  // let len = crtmp.len();

  // let mut index = 0;

  tracing::info!("测试开始");

  for item in crtmp.into_iter() {
    let mut ret = "".to_string();

    let document = match document.select(&item.name).unwrap().next() {
      Some(node) => node,
      None => {
        tracing::info!("{}---{}", "未获取到", &item.name);
        return;
      } //判断是否已经到了最后一级 当没到最后一级 这个任务就失败了 不对 没找到就应该报错了
    };

    let document = document.as_node();

    if let Some(objs) = item.obj {

      //当存在多个obj时 这里应该对下面对每个obj 进行判断 相当于for循环下有多个if 尝试考虑 过滤器
      for obj in objs {
        tracing::info!("{}", obj.name);
        ret = match obj.get_data(document) {
          Some(res) => res,
          None => "".to_string(),
        };
        //println!("{}", ret);
        tracing::info!("{}", ret);
      }
    }
  }
}
