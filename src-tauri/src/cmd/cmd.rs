use crate::model::smov::SmovFile;
use crate::serve::smov;
use crate::serve::smov_file;
use tauri::command;
use crate::response::response::Response;

//检索新文件到数据库
#[command]
pub fn query_new_file_todb() -> Response<String> {
    Response::ok(smov_file::smov_file(),"检索成功")
}

pub fn query_smov() {}

pub fn query_smov_list() {}

pub async fn retrieve_data(seek_name: String, smov_id: i64) {
    //更新数据库的seekname
    let format = seek_name;
    //检索数据
    if let Ok(res) = smov::get_test(format, smov_id).await {
        match res {
            true => println!("{}", "查找到一条数据"),
            false => println!("{}", "未找到数据"),
        }
    };
}

pub fn open_smov_win() {}

//查找所有未被检索的数据
#[command]
pub fn query_unretrieved() -> Response<Option<Vec<SmovFile>>> {
    println!("aaa");
    match SmovFile::query_db_file_id() {
        Ok(e) => return Response::new(200, Some(e), "success"),//return serde_json::to_string(&e).expect("序列化出现错误"),
        Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
    };
}

#[command]
pub fn update_seekname(id:i32,seek_name: String)->Response<Option<usize>>{
    match SmovFile::update_seekname(id,seek_name){
        Ok(e) => return Response::new(200, Some(e), "success"),
        Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
    };
    
}
