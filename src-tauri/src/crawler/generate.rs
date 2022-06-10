use std::collections::BinaryHeap;
use std::fs::write;
use tauri::command;

use crate::crawler::template::Temp;

use super::template::{Att, Corres, CrTmp, Obj};

#[command]
pub fn generate() {
  let mut heap = BinaryHeap::new();

  let mut index = 0;

  let cr_tmp = CrTmp {
    name: "#movie-list".to_string(),
    same_level: false,
    obj: None,
  };

  heap.push(cr_tmp);

  let cr_tmp = CrTmp {
    name: "#item".to_string(),
    same_level: true,
    obj: Some(vec![Obj {
      name: "a".to_string(),
      filter: None,
      can_null: false,
      types: Corres::Url,
      att: Att::Attributes("src".to_string()),
      cover: false,
    }]),
  };

  heap.push(cr_tmp);

  // for item in heap.into_iter() {
  //   println!("{}", item.name);
  // }

  let value = Temp {
    url: "https://javdb36.com".to_string(),
    version: 001,
    cr_tmp: heap,
  };

  let value = serde_json::json!(value);

  let str = serde_json::to_string_pretty(&value).unwrap();

  let path = &crate::app::APP.lock().clone().app_dir.join("model.json");

  write(&path, &str).unwrap();

  // let value: BinaryHeap<CrTmp> = serde_json::from_str(&str).unwrap();

  // println!("{:?}", value);
}
