extern crate kuchiki;
extern crate lazy_static;
extern crate reqwest;

use crate::model::smov::SmovFile;
use crate::model::smov::SmovSeek;
use kuchiki::traits::*;
use reqwest::header::HeaderMap;
use std::path::Path;
use std::{
    fs::File,
    io::{Read, Write},
};


lazy_static! {
    static ref VEC: Vec<u8> = vec![0x18u8, 0x11u8];
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
    static ref MAIN_URL: String = String::from("https://javdb.com");
}

pub async fn smov_file_bat(smov_list: &Vec<SmovFile>) {}

pub async fn get_test(format: String,id:i64) -> Result<(bool), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".parse().unwrap());
    headers.insert("cookie", "theme=auto; locale=zh; _ym_uid=1643076534203711663; _ym_d=1643076534; _ym_isad=1; over18=1; _jdb_session=u9TcLXlGGbtvm9gGwZYEpinDW9hp8wUpxrV1z88%2Bu6v7DTLIvBn9rUCQBt7O33JtmzPizK4a67uE8E75PJ56YhJQaocudrRi%2B4Ly025mTYamqzR%2FLDSfG5E%2FI32MC05KRngYkB04O%2Blli1jEvGzLLjH7GMDjERWejUQqwWtYVKEOhf2tfP7%2FPk%2BFo8Rg86S1Tai7Zg7Gc1rB0JwUqIMETFc%2BIToWoZ0jNTXWliRGSlhXpvO4Akn%2FuaBu771kG1uiSK0gQPCDTG9hheuFAjjfI0p%2FFV4b4usCkPiZZH3I2vWCM7S%2B4u6uk%2BXs--YVqvN%2Byh43AE6xyR--J5NZMl5Ko12LNJRzk%2Fzbpw%3D%3D".parse().unwrap());
    headers.insert(
        "jdsignature",
        "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
            .parse()
            .unwrap(),
    );

    let url = format!("{}/search?q={}&f=all", *MAIN_URL, format);

    let client = reqwest::Client::new();
    let res = client.get(url).headers(headers.clone()).send().await?;

    let text = res.text().await?;

    let document = kuchiki::parse_html().one(text);

    let css_selector = "#videos";

    let videos_main = document.select(css_selector).unwrap();

    let mut flag = false;

    for videos_div in videos_main {
        let videos_main = videos_div.as_node();
        let videos = videos_main.select(".grid-item").unwrap();

        for video in videos {
            let video_item = video.as_node();
            let uid = match video_item.select(".uid").unwrap().next_back(){
                Some(x) => x, 
                None => return Ok(false)// None
            };

            let name = uid.text_contents();

            let name_f = uid.text_contents().to_uppercase().replace("-", "");

            if name_f == format {
                let a = video_item.select("a").unwrap().next_back().unwrap();
                let img = video_item.select("img").unwrap().next_back().unwrap();
                let thumbs_url = img.attributes.borrow().get("data-src").unwrap().to_string();
                let att = &a.attributes;
                let href = att.borrow().get("href").unwrap().to_string(); //is_some 是否存在？
                let title = att.borrow().get("title").unwrap().to_string();
                if let Ok(res) = sava_pic(
                    &thumbs_url,
                    &(format!("thumbs_{}.jpg", name)),
                    &"E:/Pictures".to_string(),
                )
                .await
                {};
                let url = format!("{}{}", *MAIN_URL, &href);
                let res = client
                    .get(&url)
                    .headers(headers.clone())
                    .send()
                    .await?
                    .text()
                    .await?;

                println!("{}", &url);

                let document = kuchiki::parse_html().one(res);

                let video_meta_panel = document
                    .select(".video-meta-panel")
                    .unwrap()
                    .next_back()
                    .unwrap();

                let video_meta_panel = video_meta_panel.as_node();

                let smov_img = video_meta_panel
                    .select("img")
                    .unwrap()
                    .next()
                    .unwrap()
                    .attributes
                    .borrow()
                    .get("src")
                    .unwrap()
                    .to_string();

                println!("{}",smov_img);

                sava_pic(
                    &smov_img,&(format!("MAIN_{}.jpg", name)),&"E:/Pictures".to_string(),)
                .await.unwrap();

                let details = video_meta_panel.select(".panel-block").unwrap();

                let mut smov_seek = SmovSeek {
                    id,
                    name: name,
                    format: name_f,
                    release_time: String::new(),
                    duration: 0,
                    publishers: String::new(),
                    makers: String::new(),
                    series: String::new(),
                    directors: String::new(),
                    tags: Vec::new(),
                    actors: Vec::new(),
                };

                let mut flag_size = 0;

                for detail in details {
                    if flag_size == 0 {
                        // 没用部分
                        // let s = detail
                        //     .as_node()
                        //     .select("a")
                        //     .unwrap()
                        //     .next_back()
                        //     .unwrap()
                        //     .attributes
                        //     .borrow()
                        //     .get("value")
                        //     .unwrap()
                        //     .to_string();
                        // let s = detail
                        //     .as_node()
                        //     .select(".value")
                        //     .unwrap()
                        //     .next_back()
                        //     .unwrap();
                        // println!("1.{}", s.text_contents())
                    } else if flag_size == 1 {
                        let s = detail
                            .as_node()
                            .select(".value")
                            .unwrap()
                            .next_back()
                            .unwrap();
                        smov_seek.release_time = s.text_contents();
                    } else if flag_size == 2 {
                        let s = detail
                            .as_node()
                            .select(".value")
                            .unwrap()
                            .next_back()
                            .unwrap()
                            .text_contents()
                            .replace(" 分鍾", "");
                        smov_seek.duration = s.parse::<i32>().unwrap();
                    } else if flag_size == 3 {
                        let s = detail
                            .as_node()
                            .select(".value")
                            .unwrap()
                            .next_back()
                            .unwrap()
                            .text_contents();
                        smov_seek.directors = s;
                    } else if flag_size == 4 {
                        let s = detail
                            .as_node()
                            .select(".value")
                            .unwrap()
                            .next_back()
                            .unwrap()
                            .text_contents();
                        smov_seek.makers = s;
                    } else if flag_size == 5 {
                        let s = detail
                            .as_node()
                            .select(".value")
                            .unwrap()
                            .next_back()
                            .unwrap()
                            .text_contents();
                        smov_seek.publishers = s;
                    } else if flag_size == 6 {
                        let s = detail
                            .as_node()
                            .select(".value")
                            .unwrap()
                            .next_back()
                            .unwrap()
                            .text_contents();
                        smov_seek.series = s;
                    } else if flag_size == 8 {
                        let tags = detail.as_node().select("a").unwrap();

                        for tag in tags {
                            let s = tag.as_node().text_contents();
                            smov_seek.tags.push(s);
                        }
                    } else if flag_size == 9 {
                        let mut actors = detail.as_node().select("a").unwrap();
                        let actors_size = detail.as_node().select(".female").unwrap().count();
                        let mut i = 0;
                        while i != actors_size {
                            let s = actors.next().unwrap().text_contents();
                            smov_seek.actors.push(s);
                            i += 1;
                        }
                        // println!("{:?}",smov_seek);
                    }
                    flag_size = flag_size + 1;
                }
                SmovSeek::insert_by_path_name(smov_seek).unwrap();
                flag = true;
            }
        }
    }
    Ok((flag))
}

async fn sava_pic(
    url: &String,
    name: &String,
    path: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let pic_path = format!("{}/{}", path, name);
    let path = Path::new(&pic_path);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.99 Safari/537.36".parse().unwrap());
    headers.insert("cookie", "theme=auto; locale=zh; _ym_uid=1643076534203711663; _ym_d=1643076534; _ym_isad=1; over18=1; _jdb_session=u9TcLXlGGbtvm9gGwZYEpinDW9hp8wUpxrV1z88%2Bu6v7DTLIvBn9rUCQBt7O33JtmzPizK4a67uE8E75PJ56YhJQaocudrRi%2B4Ly025mTYamqzR%2FLDSfG5E%2FI32MC05KRngYkB04O%2Blli1jEvGzLLjH7GMDjERWejUQqwWtYVKEOhf2tfP7%2FPk%2BFo8Rg86S1Tai7Zg7Gc1rB0JwUqIMETFc%2BIToWoZ0jNTXWliRGSlhXpvO4Akn%2FuaBu771kG1uiSK0gQPCDTG9hheuFAjjfI0p%2FFV4b4usCkPiZZH3I2vWCM7S%2B4u6uk%2BXs--YVqvN%2Byh43AE6xyR--J5NZMl5Ko12LNJRzk%2Fzbpw%3D%3D".parse().unwrap());
    headers.insert(
        "jdsignature",
        "1639212882.lpw6vgqzsp.4c1479efddb74161f7be6bb077ac65e8"
            .parse()
            .unwrap(),
    );

    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .bytes()
        .await?;

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let content = res.bytes();
    let data: std::result::Result<Vec<_>, _> = content.collect();
    file.write_all(&data.unwrap())?;

    Ok(())
}