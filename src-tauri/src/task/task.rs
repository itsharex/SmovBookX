use parking_lot::Mutex;
use tokio_util::task::JoinMap;

#[derive(Eq, Hash, PartialEq)]
pub struct Task {
  pub uuid: String,
  pub task_type: TaskType,
  pub name: String,
}

#[derive(Eq, Hash, PartialEq)]
pub enum TaskType {
  Crawler,
  Convert,
}

//考虑在app直接对它进行创建 查看 hfs如何实现 一直在执行的 然后调两个接口出来？ 也通过消息监听 传递任务

// lazy_static! {
//   pub static ref POOL: Mutex<JoinMap<Task, bool>> = Mutex::new(new_pool());
// }

// static mut POOL: JoinMap<Task, bool> = new_pool();

// fn new_pool() -> JoinMap<Task, bool> {
//   let thread = crate::app::APP.lock().conf.thread.clone();
//   JoinMap::with_capacity(thread.try_into().unwrap())
// }

// pub fn test() {
//   let task = Task {
//     uuid: "123".to_string(),
//     task_type: TaskType::Crawler,
//     name: "fuck".to_string(),
//   };

//   POOL.spawn(task, async move { true });
// }
