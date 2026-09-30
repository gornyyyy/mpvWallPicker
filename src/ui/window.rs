use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;

use crate::ui::wallpaper_grid::{self, install_wallpaper};

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(
        r#"
        window {
            background-color: transparent;
        }
        .item {
            padding: 6px;
            border-radius: 8px;
        }
        .item.selected {
            transform: scale(1.2);
            transition: transform 20ms ease;
        }
        .preview {
            transition: transform 20ms ease;
        }
        "#,
    );
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Нет дисплея"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn build_ui(app: &Application) {
    load_css();

    let display = gtk::gdk::Display::default().expect("Нет дисплея");
    let monitor = display
        .monitors()
        .item(0)
        .and_then(|m| m.downcast::<gtk::gdk::Monitor>().ok())
        .expect("Нет монитора");

    let geometry = monitor.geometry();
    let win_width = geometry.width() as i32;
    let win_height = (geometry.height() as f64 / 2.5) as i32;

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mpvpaper Manager")
        .default_width(win_width)
        .default_height(win_height)
        .resizable(false)
        .build();

    window.set_opacity(0.95);

    let item_width = 600;

    let (grid, wallpapers) =
        wallpaper_grid::build_wallpaper_grid(&window, win_width, item_width);
    let wallpapers = Rc::new(wallpapers);
    let selected = Rc::new(RefCell::new(0usize));

    if !wallpapers.is_empty() {
        wallpaper_grid::highlight_item(0);
        wallpaper_grid::play_preview(0);
        wallpaper_grid::scroll_to(0);
    }

    let key_controller = gtk::EventControllerKey::new();
    let selected_clone = selected.clone();
    let wallpapers_clone = wallpapers.clone();
    
    let window_clone = window.clone();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        use gtk::gdk::Key;

        let mut idx = selected_clone.borrow_mut();
        let len = wallpapers_clone.len();

        if len == 0 {
            return glib::Propagation::Proceed;
        }

        match key {
            Key::Left => {
                *idx = if *idx == 0 { len - 1 } else { *idx - 1 };
                wallpaper_grid::highlight_item(*idx);
                wallpaper_grid::play_preview(*idx);
                wallpaper_grid::scroll_to(*idx);
            }
            Key::Right => {
                *idx = if *idx + 1 == len { 0 } else { *idx + 1 };
                wallpaper_grid::highlight_item(*idx);
                wallpaper_grid::play_preview(*idx);
                wallpaper_grid::scroll_to(*idx);
            }
            Key::Return | Key::KP_Enter => {
                if let Some(path) = wallpapers_clone.get(*idx) {
                    let path = path.clone();
                    if let Err(e) = install_wallpaper(&path) {
                        eprintln!("Ошибка установки: {e:#}");
                    }
                }
                window_clone.close();
            }
            Key::Escape => {
                window_clone.close();
            }
            _ => {}
        }

        glib::Propagation::Proceed
    });
    window.add_controller(key_controller);
    window.set_child(Some(&grid));
    window.present();
}