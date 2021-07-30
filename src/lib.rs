#![recursion_limit = "1024"]

mod comps;
mod menu;
mod route;

use crate::comps::updownbuttons::UpDownButtons;
use crate::comps::compiler::CCompiler;
use crate::route::Route;
use menu::Navbar;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;


#[derive(Debug)]
struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        for _ in 0..5 {
            println!("test")
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let render = Router::render(|switch: Route| match switch {
            Route::Homepage => html! {<UpDownButtons/>},
            Route::Compilerpage => html! {<CCompiler/>},
        });

        html! {
            <div class="container" height="100%">
                <Navbar/>
                <Router<Route, ()> render=render/>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    env_logger::init();
    App::<Model>::new().mount_to_body();
}
