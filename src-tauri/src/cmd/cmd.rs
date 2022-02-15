use crate::model::folder::Folder;
use crate::model::smov::SmovFile;
use crate::serve::smov;
use crate::serve::smov_file;
use tauri::command;
use crate::util::smov_format::SmovName;
use crate::response::response::Response;
use tauri::InvokeResolver;
use tauri::Manager;


//检索新文件到数据库
#[command]
pub fn query_new_file_todb() -> Response<String> {
    Response::ok(smov_file::smov_file(),"检索成功")
}

pub fn query_smov() {}

pub fn query_smov_list() {}

#[command]
pub async fn retrieve_data(window: tauri::Window,seek_name: String, smov_id: i64) -> Response<Option<i32>> {
    //更新数据库的seekname
    let format = SmovName::format_smov_name(&seek_name);

      std::thread::spawn(move || {
         tauri::async_runtime::block_on(async move {
            let _: Result<bool, anyhow::Error> = smov::get_test(format, smov_id).await;
            // InvokeResolver::new(window, String::from("123"), String::from("123"));
            // Ok(())
            println!("{}","fuck");

            //异步回调难以实现
             });
          });
     Response::new(200, Some(2), "success")
   
}

pub fn open_smov_win() {}

//查找所有未被检索的数据
#[command]
pub fn query_unretrieved() -> Response<Option<Vec<SmovFile>>> {
    match SmovFile::query_db_file_id_unseek() {
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

#[command]
pub fn insert_folder(path:String) ->Response<Option<i32>>{
    match Folder::insert_folder(path){
        Ok(e) => return Response::new(200, Some(e), "success"),
        Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
    } 
}

#[command]
pub fn query_folder() ->Response<Option<Vec<Folder>>>{
    match Folder::query_folder(){
        Ok(e) => return Response::new(200, Some(e), "success"),
        Err(err) => return Response::new(300, None, format!("{}", err).as_str()),
    } 
}
