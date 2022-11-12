mod sentances;

use iced::{Sandbox, Settings};
mod ui;

fn main() {
    ui::JapaneseSentanceApp::run(Settings {
        default_font: Some(include_bytes!("../assets/font/ZenAntique-Regular.ttf")),
        default_text_size: 30,
        ..Default::default()
    })
    .unwrap();
}
