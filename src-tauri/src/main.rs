// zip-tool-app/src-tauri/src/main.rs
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

// 圧縮：複数のファイルパスを受け取り、指定のパスに保存
#[tauri::command]
async fn compress_files(input_paths: Vec<String>, output_zip_path: String) -> Result<String, String> {
    let file = File::create(&output_zip_path).map_err(|e| e.to_string())?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for path_str in input_paths {
        let path = Path::new(&path_str);
        if !path.exists() { continue; }
        
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .ok_or("ファイル名の取得に失敗しました")?;

        zip.start_file(file_name, options).map_err(|e| e.to_string())?;
        let mut f = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        zip.write_all(&buffer).map_err(|e| e.to_string())?;
    }
    zip.finish().map_err(|e| e.to_string())?;
    Ok(format!("保存完了: {}", output_zip_path))
}

// 解凍：zipのパスを受け取り、指定のディレクトリパスに展開
#[tauri::command]
async fn extract_zip(zip_path: String, dest_dir_path: String) -> Result<String, String> {
    let file = File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = Path::new(&dest_dir_path).join(file.name());

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                std::fs::create_dir_all(p).map_err(|e| e.to_string())?;
            }
            let mut outfile = File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }
    Ok(format!("展開完了: {}", dest_dir_path))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compress_files, extract_zip])
        .run(tauri::generate_context!())
        .expect("Tauri実行中にエラーが発生しました");
}
