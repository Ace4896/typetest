#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::{Application, Settings};

mod app;

fn main() -> iced::Result {
    app::App::run(Settings::default())
}
