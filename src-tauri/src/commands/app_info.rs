use serde::Serialize;

#[derive(Serialize)]
pub struct AppInfo {
    name: String,
    version: String,
    stack: Vec<String>,
}

/// Comando de prueba: valida el pipeline Rust -> IPC -> React.
#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: String::from("ProjectLogr"),
        version: String::from(env!("CARGO_PKG_VERSION")),
        stack: vec![
            String::from("Tauri"),
            String::from("Rust"),
            String::from("React"),
        ],
    }
}
