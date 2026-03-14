use serde::Serialize;
use sysinfo::Disks;

#[derive(Serialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
}

#[derive(Serialize)]
struct DiskInfo {
    name: String,
    mount: String,
}

#[tauri::command]
fn list_disks() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();

    let mut result: Vec<DiskInfo> = disks
        .iter()
        .map(|d| DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            mount: d.mount_point().to_string_lossy().to_string(),
        })
        .collect();

    result.sort_by(|a, b| a.mount.cmp(&b.mount));

    result
}

#[tauri::command]
fn read_directory(path: String) -> Result<Vec<FileInfo>, String> {
    let entries = std::fs::read_dir(&path).map_err(|e| e.to_string())?;

    let mut result = Vec::new();

    for entry in entries {
        if let Ok(entry) = entry {
            let p = entry.path();

            result.push(FileInfo {
                name: p
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                path: p.display().to_string(),
                is_dir: p.is_dir(),
            });
        }
    }

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    builder
        .invoke_handler(tauri::generate_handler![list_disks, read_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri");
}
