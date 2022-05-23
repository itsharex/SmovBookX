use priority_queue::PriorityQueue;
use tauri::command;

use super::crawler_template::{Att, Corres, CrTmp, Obj};

#[command]
pub fn exec_test() {
  let cr_tmp = CrTmp {
    name: "test".to_string(),
    same_level: true,
    obj: vec![Obj {
      name: "test".to_string(),
      have_class: "".to_string(),
      types: Corres::Id(),
      att: Att::Text(),
    }],
  };

  let mut pq = PriorityQueue::new();

  pq.push("Apples", cr_tmp);

  println!("{}", serde_json::json!(pq));
  println!("{}", toml::to_string_pretty(&pq).unwrap());
}
