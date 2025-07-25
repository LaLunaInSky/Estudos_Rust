#[tauri::command]
pub fn verificar_menu_do_perfil(
    mostrar_o_menu_do_perfil: &str
) {
    println!(
        "O Mostrador de Menu está em {mostrar_o_menu_do_perfil}"
    );
}

