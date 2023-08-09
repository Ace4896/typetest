use iced::{
    font::{self, Font},
    Command,
};

/// The 'Noto Sans' font's bytes.
const NOTO_SANS: &'static [u8] = include_bytes!("./fonts/NotoSans/NotoSans-Regular.ttf");

/// The 'Noto Sans Mono' font's bytes.
const NOTO_SANS_MONO: &'static [u8] =
    include_bytes!("./fonts/NotoSansMono/NotoSansMono-Regular.ttf");

/// The default font to use in the app.
pub const DEFAULT: Font = Font::with_name("Noto Sans");

/// The monospace font to use in the app.
pub const MONOSPACE: Font = Font {
    family: font::Family::Name("Noto Sans Mono"),
    monospaced: true,
    ..Font::DEFAULT
};

/// Creates a command that loads the fonts used by this app.
pub fn load_command() -> Command<Result<(), font::Error>> {
    Command::batch([font::load(NOTO_SANS), font::load(NOTO_SANS_MONO)])
}
