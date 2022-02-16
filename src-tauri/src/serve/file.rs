use std::fs;
use tauri::{api::path::app_dir, Config};

pub fn _init_environment(){
    let path = app_dir(&Config::default())
    .expect("Could not get app path")
    .into_os_string()
    .into_string()
    .expect("Could not get app path");

    let path = format!("{}SmovBook",path);

    fs::create_dir_all(path).expect("初始化环境出现错误");

    //创建配置文件
}