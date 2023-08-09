#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::{Application, Settings};

mod app;
mod fonts;
mod widgets;

fn main() -> iced::Result {
    app::App::run(Settings {
        default_font: fonts::DEFAULT,
        ..Default::default()
    })
}
