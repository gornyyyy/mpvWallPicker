use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const EXTENSIONS: &[&str] = &["mp4", "gif", "webm"];

pub fn scan(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    collect(dir, &mut result)?;
    result.sort();

    Ok(result)
} 

fn collect(dir: &Path, result: &mut Vec<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Не читается {}", dir.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| {
            format!("Ошибка при чтении элемента в {}", dir.display())
        })?;
        let path = entry.path();

        if path.is_dir() {
            collect(&path, result)?;
        } else if path.is_file() && has_valid_extension(&path) {
            result.push(path);
        } 
    }
    
    Ok(())
}

fn has_valid_extension(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => EXTENSIONS.contains(&ext.to_lowercase().as_str()),
        None => false,
    }
}
