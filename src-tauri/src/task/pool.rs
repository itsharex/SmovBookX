use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use crate::{model::smov::Smov, response::response::Response};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager};
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
  pub app_handle: AppHandle,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct TaskEvent {
  pub event_type: TaskType,
  pub ask: Option<TaskAsk>,
  pub smov: Option<Smov>,
  pub status: TaskStatus,
}

struct Task<'a> {
  task_event: &'a TaskEvent,
  pub app_handle: &'a AppHandle,
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

pub type SmovPool = Arc<Mutex<TaskPool>>;

pub fn pool_new(app_handle: AppHandle) -> Result<SmovPool, PoolErr> {
  let thread_num = crate::app::APP.lock().conf.thread.clone();
  match Builder::new_multi_thread().build() {
    Ok(pool) => Ok(Arc::new(Mutex::new(TaskPool {
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
      app_handle,
    }))),
    Err(err) => Err(PoolErr::PoolCreateError(err.to_string())),
  }
}

fn pool_add_task(task_pool: SmovPool, task_ask: TaskAsk, task_type: TaskType) -> String {
  let mut task_pool_lock = task_pool.lock().unwrap();
  let uuid = Uuid::new_v4().to_string();

  let task = TaskEvent::new(task_type, task_ask).unwrap();

  task_pool_lock.tasks.insert(uuid.clone(), task);

  let task_size = task_pool_lock.exec_num.get(&TaskType::Convert).unwrap();

  if task_size < &task_pool_lock.thread_num && task_pool_lock.can_run() {
    let now_size = task_pool_lock
      .exec_num
      .get(&TaskType::Convert)
      .unwrap()
      .clone();

    task_pool_lock
      .exec_num
      .insert(TaskType::Convert, now_size + 1);

    let task_pool_copy = task_pool.clone();

    let task = task_run(task_pool_copy, uuid.clone());

    task_pool_lock.pool.spawn(task);
  }

  uuid
}

//取出数据后要解锁 不然会卡住
async fn task_run(smov_pool: SmovPool, uuid: String) {
  let mut pool = smov_pool.lock().unwrap();

  let task_event = pool.tasks.get(&uuid).unwrap().clone();
  let app_handle = &pool.app_handle;

  //生成task
  let task = Task {
    task_event: &task_event,
    app_handle,
  };

  // 解锁pool
  // Mutex::unlock(pool);

  let task_size = pool.exec_num.get(&TaskType::Convert).unwrap().clone();
  pool.exec_num.insert(TaskType::Convert, task_size - 1);
}

impl TaskPool {
  pub fn new(app_handle: AppHandle) -> Result<Self, PoolErr> {
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
        app_handle,
      }),
      Err(err) => Err(PoolErr::PoolCreateError(err.to_string())),
    }
  }

  //每一次运行都需要重新创建一个新的run 已弃用
  pub async fn run(self: &mut Self, uuid: String) {
    let mut task_evenet = self.tasks.get(&uuid).unwrap().clone();

    //执行程序

    //更新task的状态
    //task_evenet.status = task_status;
    task_evenet.status = TaskStatus::Success;
    self.tasks.insert(uuid, task_evenet);

    //判断是否有下一个task
    if let (Some(_task), true) = (self.get_next_task(), self.can_run()) {
      //给pool 塞入下一个
    } else {
      self.exec_num.insert(
        TaskType::Convert,
        self.exec_num.get(&TaskType::Convert).unwrap() - 1,
      );

      //判断是否还有正在运行的线程
      if self.get_exec_all_num() == 0 {}
    }
  }

  pub fn get_exec_all_num(self: &Self) -> i64 {
    let mut exec_num = 0;

    for (_, value) in self.exec_num.iter() {
      exec_num = exec_num + value;
    }
    exec_num
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

impl TaskEvent {}

impl Task<'_> {
  pub fn emit_status(self: &Self, task_status: TaskStatus) {
    self.app_handle.emit_all("", "").unwrap();
  }

  pub fn emit_schedule(self: &Self) {
    self.app_handle.emit_all("", "").unwrap();
  }

  pub fn join(self: &Self) {}
}

#[command]
pub fn add_task(task_pool: tauri::State<Arc<Mutex<TaskPool>>>) -> Response<Option<String>> {
  let task_pool: SmovPool = task_pool.inner().clone();
  let task_ask = TaskAsk {
    id: 1,
    name: "Test".to_string(),
  };
  Response::ok(
    Some(pool_add_task(task_pool, task_ask, TaskType::Convert)),
    "成功",
  )
}

#[command]
pub fn pause_pool(task_pool: tauri::State<Arc<Mutex<TaskPool>>>) {
  task_pool.lock().unwrap().pause();
}
