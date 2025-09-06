use dioxus::prelude::*;

mod app;

use app::App;

fn main() {
    dioxus::launch(App);
}
