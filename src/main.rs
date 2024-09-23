mod app;
mod lobby;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
