use kuchiki::{NodeData, NodeRef};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
//代表这个节点下 获取多少数据 无限递归
pub struct CrTmp {
  pub name: String,     //当前node的名称
  pub same_level: bool, //是否同级  当为同级时 当前的父node 不会被覆盖 而是复用 当前的node
  pub obj: Vec<Obj>, //当前node下 需要提取的数据 当node的数量为1 时 这个node为单独node 可以直接提取字符
}

//代表这个节点下所有的数据方法遍历数据 获取 最后的值
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Obj {
  pub name: String,       //当前node的名称 ，需要从父node提取
  pub have_class: String, //是否包含某个class
  pub types: Corres,      //提取的数据类型 最后要根据类型 插入类型
  pub att: Att, //提取数据类型 当类型为 TEXT时 直接取字符串 当为att时 取Attributes 名称参数
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Att {
  Text(),
  Attributes(String),
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
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
}

impl Obj {
  ///根据名称类型 获取数据
  pub fn get_data(self: &Self, node_ref: &NodeRef) -> Option<String> {
    let node_ref = node_ref.select(&self.name).unwrap().next_back().unwrap();
    if self.have_class != "" {
      if has_class(node_ref.as_node(), self.have_class.as_str()) {
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
    } else {
      Some(node_ref.text_contents())
    }
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
