use crate::model::smov::Smov;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_util::task::JoinMap;
use uuid::Uuid;

pub mod pool;

#[derive(Eq, Hash, PartialEq, Deserialize, Serialize, Clone)]
pub struct TaskAsk {
  pub id: i64,
  pub name: String,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct TaskEvent {
  pub uuid: String,
  pub event_type: TaskEventType,
  pub ask: Option<TaskAsk>,
  pub smov: Option<Smov>,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum TaskEventType {
  AddTask(TaskType),
  TasksLen,
  RemoveTask,
  Error(String),
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum TaskType {
  Crawler,
  Convert,
}

#[derive(Error, Debug)]
pub enum TaskErr {
  #[error("获取主数据失败")]
  _NotFound,
  #[error("unknown error")]
  _Unknown,
}

impl TaskEventType {
  fn should_smov(self: &Self) -> bool {
    self.eq(&TaskEventType::AddTask(TaskType::Convert))
      || self.eq(&TaskEventType::RemoveTask)
      || self.eq(&TaskEventType::AddTask(TaskType::Crawler))
  }
}

impl TaskEvent {
  fn _new_error(task_event_type: TaskEventType, ask: Option<&str>) -> Result<Self, TaskErr> {
    let task_ask: TaskAsk = serde_json::from_str(ask.unwrap()).unwrap();

    let smov = if task_event_type.should_smov() {
      match Smov::get_smov_by_id(task_ask.id) {
        Ok(smov) => Some(smov),
        Err(_) => return Err(TaskErr::_NotFound),
      }
    } else {
      None
    };

    Ok(TaskEvent {
      uuid: Uuid::new_v4().to_string(),
      event_type: task_event_type,
      ask: Some(task_ask),
      smov,
    })
  }
  fn new(task_event_type: TaskEventType, ask: Option<&str>) -> Self {
    let task_ask: TaskAsk = serde_json::from_str(ask.unwrap()).unwrap();

    let smov = if task_event_type.should_smov() {
      match Smov::get_smov_by_id(task_ask.id) {
        Ok(smov) => Some(smov),
        Err(err) => {
          return TaskEvent {
            uuid: Uuid::new_v4().to_string(),
            event_type: TaskEventType::Error(err.to_string()),
            ask: Some(task_ask),
            smov: None,
          }
        }
      }
    } else {
      None
    };

    TaskEvent {
      uuid: Uuid::new_v4().to_string(),
      event_type: task_event_type,
      ask: Some(task_ask),
      smov,
    }
  }
}

pub fn task_init_app(app: AppHandle) {
  let _: tauri::async_runtime::JoinHandle<()> = tauri::async_runtime::spawn(async move {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let tx_len = tx.clone();
    let thread = crate::app::APP.lock().conf.thread.clone();
    let mut pool: JoinMap<TaskEvent, bool> = JoinMap::new();

    let _ = &app.listen_global("TASK://add_task_convert", move |event| {
      if let Err(_e) = tx.send(TaskEvent::new(
        TaskEventType::AddTask(TaskType::Convert),
        event.payload(),
      )) {
        //当 这个池子出错了 只能重启了么 是否还有其他办法
        //panic!("{}", e.to_string());
      };
    });

    let _ = &app.listen_global("TASK://get_task_len", move |event| {
      if let Err(_e) = tx_len.send(TaskEvent::new(TaskEventType::TasksLen, event.payload())) {};
    });

    loop {
      let event_opt = rx.recv().await;

      if let Some(event) = event_opt {
        match event.clone().event_type{
            TaskEventType::AddTask(_) => {
              pool.spawn(event.clone(), async move {
                std::thread::sleep(std::time::Duration::from_secs(10));
                println!("{}", &event.uuid);
                true
              });
            },
            TaskEventType::TasksLen => {
              println!("{}", pool.len());
            },
            TaskEventType::RemoveTask => {},
            TaskEventType::Error(_) => {},
        }
      }
    }
  });
}
