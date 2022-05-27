use priority_queue::PriorityQueue;
use std::collections::BinaryHeap;
use std::fs::write;
use tauri::command;

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
      have_class: None,
      types: Corres::Url(),
      att: Att::Attributes("src".to_string()),
    }]),
  };

  heap.push(cr_tmp);

  // for item in heap.into_iter() {
  //   println!("{}", item.name);
  // }

  let value = serde_json::json!(heap);

  let str = serde_json::to_string_pretty(&value).unwrap();

  let path = &crate::app::APP.lock().clone().app_dir.join("model.json");

  write(&path, &str).unwrap();

  let value: BinaryHeap<CrTmp> = serde_json::from_str(&str).unwrap();

  println!("{:?}", value);
}
