use crate::model::folder::Folder;
use crate::model::smov::SmovFile;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[warn(non_upper_case_globals)]
const FILE_TYPE: &'static [&'static str] = &["mp4", "flv", "mkv"];

pub fn smov_file() -> String {
  let begin = timestamp(SystemTime::now());

  let folders = Folder::query_folder().unwrap();

  let mut smov_size = 0;

  for folder in folders {
    let main_path = folder.path;

    let loa_smov: HashSet<SmovFile> = retrieve_all(&main_path).into_iter().collect();
    // for y in loa_smov.iter() {
    //     smov_file::insert_file_data(y);
    // }

    let db_smov: HashSet<SmovFile> = SmovFile::query_db_file_unid()
      .unwrap()
      .into_iter()
      .collect();

    let smov = loa_smov.difference(&db_smov).collect::<Vec<&SmovFile>>();

    smov_size = smov_size + &smov.len();

    // for y in &smov {
    //     smov_file::insert_file_data(y).unwrap();
    // }
    SmovFile::insert_file_data(&smov).unwrap();
  }

  let end = timestamp(SystemTime::now());
  format!(
    "扫描全部文件用时:{:?}ms，共扫描到{}个差异视频文件",
    end - begin,
    &smov_size
  )
}

fn timestamp(time: SystemTime) -> i64 {
  let since_the_epoch = time
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");
  let ms = since_the_epoch.as_secs() as i64 * 1000i64
    + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
  ms
}

fn is_mov_type(extension: &String) -> bool {
  let mut flag = false;
  for val in FILE_TYPE.iter() {
    if extension.eq(val) {
      flag = true;
      break;
    }
  }
  flag
}

fn retrieve_all(path: &String) -> Vec<SmovFile> {
  let mut smovs: Vec<SmovFile> = Vec::new();

  if let Ok(entries) = fs::read_dir(&path) {
    for entry in entries {
      if let Ok(entry) = entry {
        let mat = entry.metadata().expect("无法读取文件元数据");
        if mat.is_file() {
          let realname = entry.file_name().into_string().expect("无法读取文件名称");
          let extension = match Path::new(&realname).extension() {
            Some(x) => x.to_os_string().into_string().expect("读取文件时发生错误"),
            None => String::from("None"), // None
          };

          let name = Path::new(&realname)
            .file_stem()
            .unwrap()
            .to_os_string()
            .into_string()
            .expect("读取文件时发生错误");
          let format = name.to_uppercase().replace("-C", "").replace("-", "");
          if is_mov_type(&extension) {
            let res = SmovFile {
              id: 0,
              realname: String::from(&name),
              seekname: String::from(&name),
              path: String::from(path), //对path多次引用 所以方法传入链接而不是 原始参数 在这里新建一个String 传入构造 链接就是前面带&
              len: mat.len(),
              created: timestamp(mat.created().expect("文件创建日期读取错误")),
              modified: timestamp(mat.modified().expect("文件修改日期读取错误")),
              extension,
              format,
            };
            smovs.push(res);
          }
        } else {
          let now_path = entry
            .path()
            .into_os_string()
            .into_string()
            .expect("读取文件时发生错误");
          let mut _loa_smov = retrieve_all(&now_path);
          smovs.append(&mut _loa_smov);
        }
      }
    }
  }
  smovs
}
