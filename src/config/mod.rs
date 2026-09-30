use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use kdl::KdlDocument;
use kdl::KdlValue;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn niri_config_path() -> Result<PathBuf> {
    let config = dirs::config_dir().context("Не удалось определить ~/.config")?;
    Ok(config.join("niri").join("config.kdl"))
}

pub fn update_wallpaper_path(new_wallpaper: &Path) -> Result<()> {
    let path = niri_config_path()?;
    let content = fs::read_to_string(&path)
        .with_context(|| format!("Не читается {}", path.display()))?;

    let mut doc: KdlDocument = content
        .parse()
        .with_context(|| format!("Не удалось распарсить KDL в {}", path.display()))?;

    let new_path = new_wallpaper.display().to_string();
    let mut found = false;

    for node in doc.nodes_mut() {
        // Ищем узел spawn-at-startup
        if node.name().value() != "spawn-at-startup" {
            continue;
        }

        // Проходим по всем аргументам узла
        for entry in node.entries_mut() {
            let Some(value) = entry.value().as_string() else {
                continue;
            };

            // Ищем строку, которая содержит "mpvpaper"
            if !value.contains("mpvpaper") {
                continue;
            }

            println!("Найдена строка с mpvpaper: {value}");

            // Ищем "ALL " в строке
            let Some(pos) = value.rfind("ALL ") else {
                println!("В строке нет 'ALL '");
                continue;
            };

            // Всё до "ALL " включительно — префикс
            let prefix = &value[..pos + 4]; // "mpvpaper -o '...' ALL "
            let new_value = format!("{}{}", prefix, new_path);

            println!("Новая строка: {new_value}");

            *entry.value_mut() = KdlValue::String(new_value);
            found = true;
            break;
        }

        if found {
            break;
        }
    }

    if !found {
        bail!(
            "В {} не найден spawn-at-startup с mpvpaper",
            path.display()
        );
    }

    fs::write(&path, doc.to_string())
        .with_context(|| format!("Не записывается {}", path.display()))?;

    Ok(())
}