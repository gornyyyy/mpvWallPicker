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
        if node.name().value() != "spawn-at-startup" {
            continue;
        }

        for entry in node.entries_mut() {
            let Some(value) = entry.value().as_string() else {
                continue;
            };
            
            if !value.contains("mpvpaper") {
                continue;
            }

            let Some(pos) = value.rfind("ALL ") else {
                println!("В строке нет 'ALL '");
                continue;
            };

            let prefix = &value[..pos + 4]; // "mpvpaper -o '...' ALL "
            let new_value = format!("{}{}", prefix, new_path);

            *entry.value_mut() = KdlValue::String(new_value);
            entry.clear_format();
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

    let output = doc.to_string();

    fs::write(&path, &output)
        .with_context(|| format!("Не записывается {}", path.display()))?;

    Ok(())
}