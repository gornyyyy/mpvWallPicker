use anyhow::Result;
use anyhow::bail;
use anyhow::Context;
use std::io::{self, Write};

mod core;
mod config;
mod utils;


fn main() {
    if let Err(e) = run() {
        eprintln!("Ошибка: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // Получить папку с обоями.
    let dir = utils::wallpaper_dir()?;
    // Сканировать.
    let wallpapers = core::scanner::scan(&dir)?;

    // Вывести список.
    if wallpapers.is_empty() {
        bail!("В {} нет обоев (.mp4, .gif, .webm)", dir.display());
    }

    println!("Доступные обои:");
    for (i, path) in wallpapers.iter().enumerate() {
        println!("  {}. {}", i + 1, path.file_name().unwrap().to_string_lossy());
    }

    // Прочитать выбор.
    print!("\nВыберите номер (q — выход): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "q" {
        return Ok(());
    }

    let index: usize = input.parse().context("Нужно число")?;
    let selected = wallpapers.get(index - 1).context("Неверный номер")?;

    // Остановить старый mpvpaper.
    // Запустить новый.
    // Обновить конфиг
    Ok(())
}
