use kuchiki::{NodeData, NodeRef};
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
//代表这个节点下 获取多少数据 无限递归
pub struct CrTmp {
  pub name: String,          //当前node的名称
  pub same_level: bool,      //是否同级  当为同级时 当前的父node 不会被覆盖 而是复用 当前的node
  pub obj: Option<Vec<Obj>>, //当前node下 需要提取的数据 当node的数量为1 时 这个node为单独node 可以直接提取字符
}

//代表这个节点下所有的数据方法遍历数据 获取 最后的值
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Obj {
  pub name: String,               //当前node的名称 ，需要从父node提取
  pub have_class: Option<String>, //是否包含某个class
  pub types: Corres,              //提取的数据类型 最后要根据类型 插入类型
  pub att: Att, //提取数据类型 当类型为 TEXT时 直接取字符串 当为att时 取Attributes 名称参数
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum Att {
  Text(),
  Attributes(String),
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum Corres {
  Id(),
  Name(),        //云端
  Title(),       //标题
  Format(),      //格式化后名称
  ReleaseTime(), //发行时间
  Duration(),    //时长
  Publishers(),  //方
  Makers(),      //商
  Series(),      //系列
  Directors(),   //导演
  Tags(),        //标签
  Actors(),      //演员
  Url(),         //url节点 重新获取 页面
}

impl CrTmp {
  pub fn new() -> BinaryHeap<CrTmp> {

    let path = &crate::app::APP.lock().clone().app_dir.join("model.json");

    let mut file = match File::open(&path) {
      Ok(f) => f,
      Err(e) => panic!("no such file {} exception:{}", path.to_str().unwrap(), e),
    };

    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
      Ok(s) => s,
      Err(e) => panic!("Error Reading file: {}", e),
    };

    let heap: BinaryHeap<CrTmp> = serde_json::from_str(&str_val).unwrap();

    heap
  }
}

impl Obj {
  ///根据名称类型 获取数据
  pub fn get_data(self: &Self, node_ref: &NodeRef) -> Option<String> {
    let node_ref = node_ref.select(&self.name).unwrap().next_back().unwrap();

    match &self.have_class {
      Some(class) => {
        if has_class(node_ref.as_node(), class.as_str()) {
          match &self.att {
            Att::Text() => Some(node_ref.text_contents()),
            Att::Attributes(local_name) => Some(
              node_ref
                .attributes
                .borrow()
                .get(local_name.clone())
                .unwrap()
                .to_string(),
            ),
          }
        } else {
          None
        }
      }
      None => Some(node_ref.text_contents()),
    }

    // if self.have_class.is_some() {
    //   if has_class(node_ref.as_node(), self.have_class.as_str()) {
    //     match &self.att {
    //       Att::Text() => Some(node_ref.text_contents()),
    //       Att::Attributes(local_name) => Some(
    //         node_ref
    //           .attributes
    //           .borrow()
    //           .get(local_name.clone())
    //           .unwrap()
    //           .to_string(),
    //       ),
    //     }
    //   } else {
    //     None
    //   }
    // } else {
    // }
  }
}

pub fn has_class(el: &NodeRef, class: &str) -> bool {
  let data = match el.data() {
    NodeData::Element(data) => data,
    _ => return false,
  };

  let attributes = data.attributes.borrow();

  if let Some(class_attr) = attributes.get("class") {
    class_attr.split_whitespace().any(|piece| piece == class)
  } else {
    false
  }
}
