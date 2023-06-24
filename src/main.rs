mod app;

pub mod components;
pub mod views;
pub mod utils;

use app::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
