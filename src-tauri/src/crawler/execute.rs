use crate::crawler::template::CrTmp;
use parking_lot::Mutex;
use std::collections::BinaryHeap;
use tauri::command;

lazy_static! {
  pub static ref MODEL: BinaryHeap<CrTmp> = CrTmp::new();
}

#[command]
pub async fn execute() {
  println!("{:?}", *MODEL);
}
