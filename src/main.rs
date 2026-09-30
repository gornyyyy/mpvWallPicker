mod core;
mod config;
mod utils;
mod app;
mod ui;

use gtk::prelude::*;
use gtk::{Application, glib};

const APP_ID: &str = "org.egor.MpvpaperManager";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        ui::window::build_ui(app);
    });

    app.run()
}


