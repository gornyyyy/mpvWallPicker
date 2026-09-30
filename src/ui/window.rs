use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use crate::ui::wallpapers_grid;

pub fn build_ui(app: &Application) {
    let display = gtk::gdk::Display::default().expect("Нет дисплея");
    let monitor = display
        .monitors()
        .item(0)
        .and_then(|m| m.downcast::<gtk::gdk::Monitor>().ok())
        .expect("Нет монитора");

    let geometry = monitor.geometry();
    let win_width = (geometry.width() as f64 * 0.6) as i32;
    let win_height = (geometry.height() as f64 / 3.0) as i32;

    // Главное окно
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Mpvpaper Manager")
        .default_width(win_width)
        .default_height(win_height)
        .resizable(false)
        .build();

    // Сетка обоев
    let grid = wallpapers_grid::build_wallpaper_grid(&window);
    window.set_child(Some(&grid));

    window.present();
}