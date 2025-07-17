use tauri::ipc::Response;

#[tauri::command]
pub fn read_file() -> Response {
    let data = std::fs::read("path/to/file").unwrap();

    Response::new(data)
}

#[tauri::command]
pub fn my_custom_command() {
    println!("\nI was invoked from TypeScript\n");
}

#[tauri::command(rename_all = "snake_case")]
pub fn my_custom_command_2(your_name: String) -> String {
    format!(
        "Hello {}!",
        your_name
    )
}