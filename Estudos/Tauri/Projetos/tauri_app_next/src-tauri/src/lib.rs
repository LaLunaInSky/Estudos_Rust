#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command]
fn verificar_menu_do_perfil(menu_aberto: bool) {
  println!("O menu está {}", menu_aberto);
}

pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![verificar_menu_do_perfil])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
