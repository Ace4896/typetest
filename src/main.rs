#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use iced::Application;

mod app;

fn main() -> iced::Result {
    app::App::run(iced::Settings::default())
}
