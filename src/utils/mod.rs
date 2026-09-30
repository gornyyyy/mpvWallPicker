use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};

pub fn wallpaper_dir() -> Result<PathBuf> {
    let pictures = dirs::video_dir().context("Не удалось найти ~/Videos")?;
    let wallpapers = pictures.join("Wallpapers");

    fs::create_dir_all(&wallpapers)
        .with_context(|| format!("Не удалось создать {}", wallpapers.display()))?;
        
    Ok(wallpapers)
}

pub fn read_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush().context("Не удалось сбросить stdout")?;

    let mut input= String::new();
    io::stdin().read_line(&mut input).context("Не удалось прочитать stdin")?;

    Ok(input.trim().to_string())
}