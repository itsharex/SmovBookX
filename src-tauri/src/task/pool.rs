use std::{collections::HashMap, sync::Mutex};

use crate::{model::smov::Smov, response::response::Response};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle};
use thiserror::Error;
use tokio::runtime::{Builder, Runtime};
use uuid::Uuid;

// 将线程池 注入tauri 使用commond 操作 不知道行不行
// 注入参考 https://github.com/tauri-apps/tauri/discussions/4514
// hashMap 参考 https://www.jianshu.com/p/4b078acba28d
// 按照当前写法 https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html 也能实现
// 考虑用lazystatic存 线程数等
// https://stackoverflow.com/questions/26199926/how-to-terminate-or-suspend-a-rust-thread-from-another-thread 线程停止等资料
pub struct TaskPool {
  pub pool: Runtime,
  pub tasks: HashMap<String, TaskEvent>,
  pub exec_num: HashMap<TaskType, i64>,
  pub thread_num: i64,
  pub status: PoolStatus,
  pub app_handle: Option<AppHandle>,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct TaskEvent {
  pub event_type: TaskType,
  pub ask: Option<TaskAsk>,
  pub smov: Option<Smov>,
  pub status: TaskStatus,
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

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum TaskStatus {
  Wait,    //等待
  Running, //正在运行
  Fail,    //失败
  Success, //成功
}

#[derive(Error, Debug)]
pub enum PoolErr {
  #[error("线程池创建失败！")]
  PoolCreateError(String),
}

#[derive(Error, Debug)]
pub enum TaskErr {
  #[error("获取主数据失败")]
  _NotFound,
  #[error("unknown error")]
  _Unknown,
}

impl TaskPool {
  pub fn new(app_handle: Option<AppHandle>) -> Result<Self, PoolErr> {
    let thread_num = crate::app::APP.lock().conf.thread.clone();
    match Builder::new_multi_thread().build() {
      Ok(pool) => Ok(TaskPool {
        pool,
        tasks: HashMap::new(),
        exec_num: {
          let mut map = HashMap::new();
          map.insert(TaskType::Convert, 0);
          map.insert(TaskType::Crawler, 0);
          map
        },
        thread_num,
        status: PoolStatus::Idle,
        app_handle: app_handle,
      }),
      Err(err) => Err(PoolErr::PoolCreateError(err.to_string())),
    }
  }
  pub fn add_task(self: &mut Self, task_ask: TaskAsk, task_type: TaskType) -> String {
    //判断当前是否还有空余线程
    let task_size = self.exec_num.get(&TaskType::Convert).unwrap();

    let uuid = Uuid::new_v4().to_string();

    let task = TaskEvent::new(task_type, task_ask).unwrap();

    self.tasks.insert(uuid.clone(), task);

    //当有空闲线程且 状态为空闲时 自动调用run 否则直接返回 因为在run里 会自动调用下一条
    if task_size < &self.thread_num && self.can_run() {
      //更新当前类型线程进程数
      self.exec_num.insert(
        TaskType::Convert,
        self.exec_num.get(&TaskType::Convert).unwrap() + 1,
      );
      //run 会阻塞 要放入异步运行时 但是这里有个问题 我的这个pool 不能在里面调pool 咋办呢 凉拌 不知道 再说把
      self.pool.spawn(async move {
        //self.run(uuid);
      });
    }

    uuid
  }

  //每一次运行都需要重新做一个新的run
  pub async fn run(self: &mut Self, uuid: String) {
    let mut task_evenet = self.tasks.get(&uuid).unwrap().clone();

    //执行程序

    // 更新task的状态
    //task_evenet.status = task_status;
    task_evenet.status = TaskStatus::Success;
    self.tasks.insert(uuid, task_evenet);

    //判断是否有下一个task
    if let (Some(_task), true) = (self.get_next_task(), self.can_run()) {
      //给pool 塞入下一个
    } else {
      //判断是否还有正在运行的线程
      let mut exec_num = 0;

      for (_, value) in self.exec_num.iter() {
        exec_num = exec_num + value;
      }

      self.exec_num.insert(
        TaskType::Convert,
        self.exec_num.get(&TaskType::Convert).unwrap() - 1,
      );

      if exec_num == 0 {}
    }
  }

  pub fn get_next_task(self: &Self) -> Option<TaskEvent> {
    for (_, value) in self.tasks.iter() {
      if value.status == TaskStatus::Wait {
        return Some(value.clone());
      }
    }
    None
  }

  pub fn pause(self: &mut Self) {
    self.status = PoolStatus::Pause;
  }

  pub fn can_run(self: &Self) -> bool {
    self.status.eq(&PoolStatus::Idle) || self.status.eq(&PoolStatus::Running)
  }
}

impl TaskEvent {
  fn new(task_type: TaskType, task_ask: TaskAsk) -> Result<Self, TaskErr> {
    let smov = match Smov::get_smov_by_id(task_ask.id) {
      Ok(smov) => Some(smov),
      Err(_) => return Err(TaskErr::_NotFound),
    };

    Ok(TaskEvent {
      event_type: task_type,
      ask: Some(task_ask),
      smov,
      status: TaskStatus::Wait,
    })
  }
}

#[command]
pub fn add_task(task_pool: tauri::State<Mutex<TaskPool>>) -> Response<Option<String>> {
  let uuid = task_pool.lock().unwrap().add_task(
    TaskAsk {
      id: 1,
      name: "TEST".to_string(),
    },
    TaskType::Convert,
  );

  Response::ok(Some(uuid), "成功")
}

#[command]
pub fn pause_pool(task_pool: tauri::State<Mutex<TaskPool>>) {
  task_pool.lock().unwrap().pause();
}
