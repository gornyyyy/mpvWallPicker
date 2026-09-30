use gtk::prelude::*;
use gtk::{
    gio, Box as GtkBox, Label, MediaFile, Orientation, Picture, PolicyType,
    ScrolledWindow,
};
use std::cell::RefCell;
use std::path::PathBuf;

use crate::config;
use crate::core::{mpvpaper, scanner};
use crate::utils;

thread_local! {
    static ITEMS: RefCell<Vec<GtkBox>> = RefCell::new(Vec::new());
    static PREVIEWS: RefCell<Vec<MediaFile>> = RefCell::new(Vec::new());
    static HADJUSTMENT: RefCell<Option<gtk::Adjustment>> = RefCell::new(None);
}

pub fn build_wallpaper_grid(
    _parent_window: &gtk::ApplicationWindow,
    win_width: i32,
    item_width: i32,
) -> (ScrolledWindow, Vec<PathBuf>) {
    let side_margin = ((win_width - item_width) / 2).max(0);

    let hbox = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(65)
        .margin_top(12)
        .margin_bottom(12)
        .vexpand(true)
        .build();

    let left_spacer = GtkBox::builder()
        .width_request(side_margin-65)
        .build();
    hbox.append(&left_spacer);

    let mut paths = Vec::new();

    match load_wallpapers() {
        Ok(wallpapers) if !wallpapers.is_empty() => {
            for path in wallpapers {
                let item = build_wallpaper_item(&path);
                hbox.append(&item);
                ITEMS.with(|items| items.borrow_mut().push(item));
                paths.push(path);
            }
        }
        Ok(_) => {
            hbox.append(&Label::new(Some("Нет обоев в папке")));
        }
        Err(e) => {
            eprintln!("Ошибка загрузки обоев: {e:#}");
            hbox.append(&Label::new(Some("Ошибка загрузки обоев")));
        }
    }

    let right_spacer = GtkBox::builder()
        .width_request(side_margin)
        .build();
    hbox.append(&right_spacer);

    let scrolled = ScrolledWindow::builder()
        .child(&hbox)
        .hscrollbar_policy(PolicyType::External)
        .vscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .hexpand(true)
        .build();

    let hadj = scrolled.hadjustment();
    HADJUSTMENT.with(|h| *h.borrow_mut() = Some(hadj));

    (scrolled, paths)
}

pub fn highlight_item(index: usize) {
    ITEMS.with(|items| {
        let items = items.borrow();
        for (i, item) in items.iter().enumerate() {
            if i == index {
                item.add_css_class("selected");
            } else {
                item.remove_css_class("selected");
            }
        }
    });
}

pub fn play_preview(index: usize) {
    PREVIEWS.with(|previews| {
        let previews = previews.borrow();
        for (i, stream) in previews.iter().enumerate() {
            stream.set_playing(i == index);
        }
    });
}

pub fn scroll_to(index: usize) {
    ITEMS.with(|items| {
        let items = items.borrow();
        let Some(item) = items.get(index) else { return };

        let width = item.width() as f64;
        let x = item.allocation().x() as f64;

        HADJUSTMENT.with(|h| {
            if let Some(adj) = h.borrow().as_ref() {
                let page_size = adj.page_size();
                let target = x - (page_size - width) / 2.0;
                adj.set_value(target.max(0.0));
            }
        });
    });
}

fn load_wallpapers() -> anyhow::Result<Vec<PathBuf>> {
    let dir = utils::wallpaper_dir()?;
    scanner::scan(&dir)
}

fn build_wallpaper_item(path: &PathBuf) -> GtkBox {
    let container = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .css_classes(["item"])
        .vexpand(true)
        .build();

    let picture = Picture::builder()
        .width_request(600)
        .vexpand(true)
        .css_classes(["preview"])
        .build();

    let file = gio::File::for_path(path);
    let media_stream = MediaFile::for_file(&file);
    media_stream.set_loop(true);
    media_stream.set_playing(false);

    picture.set_paintable(Some(&media_stream));
    PREVIEWS.with(|previews| previews.borrow_mut().push(media_stream));

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "?".to_string());
    let label = Label::new(Some(&name));
    label.set_ellipsize(gtk::pango::EllipsizeMode::Middle);
    label.set_max_width_chars(25);

    container.append(&picture);
    container.append(&label);
    container
}

pub fn install_wallpaper(path: &PathBuf) -> anyhow::Result<()> {
    mpvpaper::stop();
    mpvpaper::start(path)?;
    config::update_wallpaper_path(path)?;
    Ok(())
}