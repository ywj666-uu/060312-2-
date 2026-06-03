mod app;
mod api;
mod types;
mod components;
mod pages;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
