use kuchiki::{NodeData, NodeRef};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Deserialize, Serialize, Debug)]
//类型
pub struct Temp {
  pub url: String,
  pub version: u8,
  pub cr_tmp: BinaryHeap<CrTmp>,
}

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
  pub name: String,           //当前node的名称 ，需要从父node提取
  pub filter: Option<Filter>, //过滤器
  pub can_null: bool,
  pub types: Corres, //提取的数据类型 最后要根据类型 插入类型
  pub att: Att,      //提取数据类型 当类型为 TEXT时 直接取字符串 当为att时 取Attributes 名称参数
  pub cover: bool,   //是否覆盖node 将当前的objNode作为下一级的node
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Filter {
  pub name: String,      //过滤条件
  pub types: FilterType, //过滤类型
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum Att {
  Text,
  Attributes(String),
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum FilterType {
  Class,           //class 条件时判断为当前的doc是否存在这个class 如果没有 则跳出
  Element(String), //element 条件时判断为当前doc下获取 名称为 这个值的node 内部的字符串是否与 判断相等
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum Corres {
  Id,
  Name,        //云端
  Title,       //标题
  Format,      //格式化后名称
  ReleaseTime, //发行时间
  Duration,    //时长
  Publishers,  //方
  Makers,      //商
  Series,      //系列
  Directors,   //导演
  Tags,        //标签
  Actors,      //演员
  Url,         //url节点 重新获取 页面
}

impl CrTmp {
  pub fn new() -> BinaryHeap<CrTmp> {
    //判断是否存在文件 当文件不存在时 拉取文件 当文件存在时直接获取文件中的数据

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

impl Temp {
  pub fn new() -> Temp {
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

    let heap: Temp = serde_json::from_str(&str_val).unwrap();

    heap
  }
}

impl Obj {
  ///根据名称类型 获取数据
  pub fn get_data(self: &Self, mut node_refs: NodeRef) -> Option<String> {
    //不能next_back 应该遍历这些 元素 然后返回需要的数据 虽然对于一个元素下有多个元素的项目会出现复杂度加倍的情况 但是数据量其实不大 还好
    //遍历节点 返回第一个值
    //返回数据更改 返回一个可能存在的对应域 这个域会作为接下来的顶级域

    // 这个逻辑太复杂了 晕了

    for node_ref in node_refs.select(&self.name).unwrap() {
      if self.cover {
        let nod= node_ref.as_node().clone() ;
        node_refs = nod;
      }
      if let Some(filter) = &self.filter {
        match &filter.types {
          FilterType::Class => {
            if has_class(node_ref.as_node(), filter.name.as_str()) {
              return match &self.att {
                Att::Text => Some(node_ref.text_contents()),
                Att::Attributes(local_name) => Some(
                  node_ref
                    .attributes
                    .borrow()
                    .get(local_name.clone())
                    .unwrap()
                    .to_string(),
                ),
              };
            }
          }
          FilterType::Element(name) => {
            match node_ref.as_node().select(&filter.name).unwrap().next() {
              Some(node) => {
                if node.text_contents().eq(name) {
                  return match &self.att {
                    Att::Text => Some(node_ref.text_contents()),
                    Att::Attributes(local_name) => Some(
                      node_ref
                        .attributes
                        .borrow()
                        .get(local_name.clone())
                        .unwrap()
                        .to_string(),
                    ),
                  };
                }
              }
              None => {}
            };
          }
        }
      } else {
        return match &self.att {
          Att::Text => Some(node_ref.text_contents()),
          Att::Attributes(local_name) => {
            tracing::info!("{:?}", node_ref);
            match node_ref.attributes.borrow().get(local_name.clone()) {
              Some(res) => Some(res.to_string()),
              None => None,
            }
          }
        };
      }
    }
    None
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
