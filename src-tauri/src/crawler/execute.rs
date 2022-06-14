use crate::crawler::template::{Corres, Temp};
use axum::http::HeaderMap;
use kuchiki::traits::TendrilSink;
use parking_lot::Mutex;
use tauri::command;

lazy_static! {
  pub static ref MODEL: Mutex<Temp> = Mutex::new(Temp::new());
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

  //lazy_static::initialize(&MODEL);
  //假设出现 互斥锁出现错误的情况 好像可以重新访问 tm这个概念太难懂了 以后再说

  let model = MODEL.lock().clone();

  let crtmp = model.cr_tmp.clone();

  tracing::info!("{:?}", crtmp);

  let main_url = model.url.clone();

  let format = "MIAA213";

  //注入过滤器
  model.injection_name(String::from(format));

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

  //tracing::info!("{}", text);

  let mut document = kuchiki::parse_html().one(text);

  // let len = crtmp.len();

  // let mut index = 0;

  tracing::info!("测试开始");

  for item in crtmp.into_iter() {
    tracing::info!("{}",item.name);
    document = match !item.same_level {
      true => document,
      false => {
        let node = match document.select(&item.name).unwrap().next() {
          Some(node) => node,
          None => {
            tracing::info!("{}---{}", "未获取到", &item.name);
            return;
          } //判断是否已经到了最后一级 当没到最后一级 这个任务就失败了 不对 没找到就应该报错了
        };
        node.as_node().clone()
      }
    };

    if let Some(objs) = item.obj {
      //当存在多个obj时 这里应该对下面对每个obj 进行判断 相当于for循环下有多个if 尝试考虑 过滤器
      //这里应该要对 name类型有个判断 只有name匹配到才会进行下一步 一直没获取到的话直接报错 那name的优先级应该在开始 设置同级的标志不继续向下获取
      //那这个同级name可以随便取 下一级获取下一个document 的时候再取一样的
      //所以每一个Crtmp应该要唯一 obj可能是多个元素
      //name类型应该需要匹配多个域并返回当前的域 所以gatdata里 可能需要对当前的域做一个返回 作为下一个域

      //需要解决两个很大的问题
      //问题1 如何优雅的做到名称的匹配 并返回当前这个域下的字符串
      //问题2 如何优雅的做到这个区域内某个字符串是否存在的判断 解决办法：在obj中添加 过滤器的字段
      //所有的crtmp都应该作为一个父域来实现 这个父域应当唯一！ crtmp是流程 是获取父 obj是所有的元素获取
      //如何返回对应的下级obj是一个问题 或者说我不需要返回 因为假设匹配到数据后 必定存在在第一个 所以我完全可以直接获取第一个元素
      //但是我是打算以后都可以用这个模板的 所以可能还是得弄一个返回类型
      for obj in objs {
        tracing::info!("{}", obj.name);
        let ret = match obj.get_data(&document) {
          Some(res) => {
            //判断是否覆盖当前doc
            if obj.cover {
              document = res.node.clone();
            }
            tracing::info!("为啥没有返回？？？{}", res.res);
            res
          }
          None => {
            //判断是否可以为空
            match obj.can_null {
              true => continue,
              false => {
                tracing::info!("{}---{}", "未获取到", &obj.name);
                return; //这里应该要报错
              }
            }
          }
        };

        //类型判断
        //特殊类型 name url
        if obj.types == Corres::Name {
          if ret.res.to_uppercase().replace("-", "") == format {};
        }

        tracing::info!("{:?}", ret.res);
      }
    }
  }
}
