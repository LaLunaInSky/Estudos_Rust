#[tauri::command]
pub fn verificar_menu_do_perfil(
    mensagem: &str
) {
    println!(
        "O Mostrador de Menu está em {mensagem}"
    );
}

