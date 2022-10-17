use std::{collections::HashMap, sync::Mutex, thread::sleep};

use crate::model::smov::Smov;
use rand::Rng;
use rayon::ThreadPool;
use serde::{Deserialize, Serialize};
use tauri::command;
use thiserror::Error;
use tokio::sync::mpsc;

// 将线程池 注入tauri 使用commond 操作 不知道行不行
// 注入参考 https://github.com/tauri-apps/tauri/discussions/4514
// hashMap 参考 https://www.jianshu.com/p/4b078acba28d
// 按照当前写法 https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html 也能实现 
// 考虑用lazystatic存 线程数等
// https://stackoverflow.com/questions/26199926/how-to-terminate-or-suspend-a-rust-thread-from-another-thread 线程停止等资料
pub struct TaskPool {
  pub pool: ThreadPool,
  pub tasks: Vec<TaskEvent>,
  pub exec_num: HashMap<TaskType, i32>,
  pub thread_num: i64,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct TaskEvent {
  pub uuid: String,
  pub event_type: TaskType,
  pub ask: Option<TaskAsk>,
  pub smov: Option<Smov>,
}

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Clone)]
pub struct TaskAsk {
  pub id: i64,
  pub name: String,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum TaskType {
  Crawler,
  Convert,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum PoolStatus {
  Running, //正在运行
  Pause,   //暂停
  Idle,    //空闲
}

#[derive(Error, Debug)]
pub enum PoolErr {
  #[error("获取主数据失败")]
  PoolCreateError(String),
}

impl TaskPool {
  pub fn new() -> Result<Self, PoolErr> {
    let thread_num = crate::app::APP.lock().conf.thread.clone();
    let mut ss = 100;
    match rayon::ThreadPoolBuilder::new()
      .num_threads((thread_num * 2).try_into().unwrap())
      .build()
    {
      Ok(pool) => Ok(TaskPool {
        pool,
        tasks: vec![],
        exec_num: {
          let mut map = HashMap::new();
          map.insert(TaskType::Convert, 0);
          map.insert(TaskType::Crawler, 0);
          map
        },
        thread_num,
      }),
      Err(err) => Err(PoolErr::PoolCreateError(err.to_string())),
    }
  }
  pub fn add_task(self: &mut Self) {
    //插入tasks中 当当前状态为 空闲 自动调用 run
    self.exec_num.insert(
      TaskType::Convert,
      self.exec_num.get(&TaskType::Convert).unwrap() + 1,
    );
    self.run();
    println!("{}", self.exec_num.get(&TaskType::Convert).unwrap());
  }
  pub fn run(self: &mut Self) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    //执行程序
    let a = self.exec_num.get(&TaskType::Convert).unwrap();
    self.pool.spawn(move || {
      let rnd = rand::thread_rng().gen_range(1..=10);
      sleep(std::time::Duration::from_secs(rnd));
      tx.send(()).unwrap();
    });
    //判断这个类型是否还有空位当前线程池是否还有位置 如果有位置 加入下一条
    let s = rx.recv();
  }
  pub async fn pause(self: &mut Self) {
    //执行程序

    //判断这个类型是否还有空位当前线程池是否还有位置 如果有位置 加入下一条
  }
}

#[command]
pub fn add_task(task_pool: tauri::State<Mutex<TaskPool>>) {
  task_pool.lock().unwrap().add_task();
}
