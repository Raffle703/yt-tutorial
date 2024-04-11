mod app;
use app::App;
mod board;

fn main() {
    yew::Renderer::<App>::new().render();
}
