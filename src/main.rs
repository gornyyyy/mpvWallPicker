use anyhow::Result;
use anyhow::bail;
use std::path::Path;
use std::path::PathBuf;

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

    loop {
        // Сканировать.
        let wallpapers = core::scanner::scan(&dir)?;

        // Вывести список.
        if wallpapers.is_empty() {
            bail!("В {} нет обоев (.mp4, .gif, .webm)", dir.display());
        }

        print_wallpapers(&wallpapers, &dir)?;

        // Прочитать выбор.
        let input = utils::read_line("\nВыберите номер (q - выход, r - обновить): ")?;
        
        if input == "q" {
            break;
        } else if  input == "r" {
            continue;
        }

        let Some(index) = parse_index(&input) else {
            println!("Нужно ввести число, а не «{input}»");
            continue;
        };

        let Some(selected) = get_wallpaper(&wallpapers, index) else {
            println!("Номер должен быть от 1 до {}", wallpapers.len());
            continue;
        };
        
        // Остановить старый mpvpaper.
        core::mpvpaper::stop();
        // Запустить новый.
        core::mpvpaper::start(selected)?;

        // Обновить конфиг
        config::update_wallpaper_path(selected)?;
    }
    Ok(())
}

fn print_wallpapers(wallpapers: &[PathBuf], dir: &Path) -> Result<()> {
    print!("\x1B[2J\x1B[1;1H");
    if wallpapers.is_empty() {
        bail!("В {} нет обоев (.mp4, .gif, .webm)", dir.display());
    }

    println!("Доступные обои:");
    for (i, path) in wallpapers.iter().enumerate() {
        println!("  {}. {}", i + 1, path.file_name().unwrap().to_string_lossy());
    }

    Ok(())
}

fn parse_index(input: &str) -> Option<usize> {
    let n: usize = input.parse().ok()?;
    if n == 0 {
        return None;
    }
    Some(n)
}

fn get_wallpaper(wallpapers: &[PathBuf], index: usize) -> Option<&PathBuf> {
    wallpapers.get(index - 1)
}
