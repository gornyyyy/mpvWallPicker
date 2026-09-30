use gtk::prelude::*;
use gtk::glib;
use gtk::{
    AlertDialog, Box as GtkBox, FlowBox, Label, Orientation, Picture, PolicyType,
    ScrolledWindow,
};
use std::path::PathBuf;

use crate::config;
use crate::core::{mpvpaper, scanner};
use crate::utils;

pub fn build_wallpaper_grid(parent_window: &gtk::ApplicationWindow) -> ScrolledWindow {
    let flow_box = FlowBox::builder()
        .orientation(Orientation::Horizontal)
        .selection_mode(gtk::SelectionMode::None)
        .max_children_per_line(20)
        .min_children_per_line(1)
        .row_spacing(12)
        .column_spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    match load_wallpapers() {
        Ok(wallpapers) if !wallpapers.is_empty() => {
            for path in wallpapers {
                let item = build_wallpaper_item(&path, parent_window);
                flow_box.append(&item);
            }
        }
        Ok(_) => {
            flow_box.append(&Label::new(Some("Нет обоев в папке")));
        }
        Err(e) => {
            eprintln!("Ошибка загрузки обоев: {e:#}");
            flow_box.append(&Label::new(Some("Ошибка загрузки обоев")));
        }
    }

    ScrolledWindow::builder()
        .child(&flow_box)
        .hscrollbar_policy(PolicyType::Automatic)
        .vscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .hexpand(true)
        .build()
}

fn load_wallpapers() -> anyhow::Result<Vec<PathBuf>> {
    let dir = utils::wallpaper_dir()?;
    scanner::scan(&dir)
}

fn build_wallpaper_item(
    path: &PathBuf,
    parent_window: &gtk::ApplicationWindow,
) -> GtkBox {
    let container = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();

    let picture = Picture::builder()
        .width_request(200)
        .height_request(120)
        .build();

    // --- Превью в фоне ---
    let path_for_thumb = path.clone();
    let picture_clone = picture.clone();
    let (sender, receiver) = async_channel::bounded::<anyhow::Result<PathBuf>>(1);

    std::thread::spawn(move || {
        let result = generate_thumbnail(&path_for_thumb);
        let _ = sender.send_blocking(result);
    });

    glib::MainContext::default().spawn_local(async move {
        if let Ok(result) = receiver.recv().await {
            match result {
                Ok(thumb_path) => picture_clone.set_filename(Some(&thumb_path)),
                Err(e) => eprintln!("Превью не создано: {e}"),
            }
        }
    });

    // Имя файла
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "?".to_string());
    let label = Label::new(Some(&name));
    label.set_ellipsize(gtk::pango::EllipsizeMode::Middle);
    label.set_max_width_chars(25);

    // --- Кнопка установки ---
    let button = gtk::Button::builder().label("Установить").build();

    let path_for_click = path.clone();
    let window_clone = parent_window.clone();
    button.connect_clicked(move |_| {
        let path = path_for_click.clone();
        let window = window_clone.clone();

        let (sender, receiver) = async_channel::bounded::<anyhow::Result<()>>(1);

        std::thread::spawn(move || {
            let result = install_wallpaper(&path);
            let _ = sender.send_blocking(result);
        });

        glib::MainContext::default().spawn_local(async move {
            if let Ok(result) = receiver.recv().await {
                let message = match result {
                    Ok(_) => "Обои установлены".to_string(),
                    Err(e) => format!("Ошибка: {e:#}"),
                };
                let dialog = AlertDialog::builder()
                    .modal(true)
                    .message(message)
                    .build();
                dialog.show(Some(&window));
            }
        });
    });

    container.append(&picture);
    container.append(&label);
    container.append(&button);
    container
}

fn generate_thumbnail(video: &PathBuf) -> anyhow::Result<PathBuf> {
    use anyhow::Context;
    use std::process::Command;

    let thumb_dir = std::env::temp_dir().join("mpvpaperManager_thumbs");
    std::fs::create_dir_all(&thumb_dir).context("Не создать папку превью")?;

    let file_stem = video
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "thumb".to_string());
    let thumb_path = thumb_dir.join(format!("{file_stem}.png"));

    if thumb_path.exists() {
        return Ok(thumb_path);
    }

    let status = Command::new("ffmpeg")
        .args(["-ss", "00:00:01", "-i"])
        .arg(video)
        .args(["-frames:v", "1", "-vf", "scale=200:-1", "-y"])
        .arg(&thumb_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .context("ffmpeg не запустился")?;

    if !status.success() {
        anyhow::bail!("ffmpeg вернул ошибку");
    }
    Ok(thumb_path)
}

fn install_wallpaper(path: &PathBuf) -> anyhow::Result<()> {
    mpvpaper::stop();
    mpvpaper::start(path)?;
    config::update_wallpaper_path(path)?;
    Ok(())
}