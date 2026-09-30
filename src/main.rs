mod core;
mod config;
mod utils;

use anyhow::Result;

fn main() {
    if let run Err(e) = run() {
        eprintln!("Ошибка: {e:#}");
        std::process:exit(1);
    }
    Ok(())
}

fn run() -> Result {
    // Получить папку с обоями.
    // Сканировать.
    // Вывести список.
    // Прочитать выбор.
    // Остановить старый mpvpaper.
    // Запустить новый.
    // Обновить конфиг
    Ok(())
}
