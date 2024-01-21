use app::App;
use iced::{
    wayland::{
        actions::layer_surface::SctkLayerSurfaceSettings,
        layer_surface::{Anchor, KeyboardInteractivity, Layer},
        InitialSurface,
    },
    window::Id,
    Application, Font, Pixels, Settings,
};

mod app;
mod centerbox;
mod components;
mod menu;
mod modules;
mod style;
mod utils;

#[tokio::main]
async fn main() {
    let height = 34;

    App::run(Settings {
        antialiasing: true,
        exit_on_close_request: false,
        initial_surface: InitialSurface::LayerSurface(SctkLayerSurfaceSettings {
            id: Id::MAIN,
            keyboard_interactivity: KeyboardInteractivity::None,
            namespace: "ashell2".into(),
            layer: Layer::Top,
            size: Some((None, Some(height))),
            anchor: Anchor::TOP.union(Anchor::LEFT).union(Anchor::RIGHT),
            exclusive_zone: height as i32,
            ..Default::default()
        }),
        flags: (),
        id: None,
        default_font: Font::default(),
        fonts: Default::default(),
        default_text_size: Pixels::from(14.),
    })
    .unwrap();
}
