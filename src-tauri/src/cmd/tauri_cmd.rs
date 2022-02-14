use serde::Deserialize;
use std::process::Command;

use tauri::command;

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    // your custom commands
    // multiple arguments are allowed
    // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
    MyCustomCommand { argument: String },
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RequestBody {
    id: i32,
    name: String,
}

#[command]
pub fn log_operation(event: String, payload: Option<String>) {
    println!("{} {:?}", event, payload);
}

#[command]
pub fn perform_request(endpoint: String, body: RequestBody) -> String {
    println!("{} {:?}", endpoint, body);
    "message response".into()
}

//在资源管理器中打开
#[command]
pub fn open_in_explorer(path: String) {
    Command::new( "explorer" )
        .arg( "." ) // <- Specify the directory you'd like to open.
        .spawn( )
        .unwrap( );
}
