#![recursion_limit = "1024"]

mod comps;
mod menu;
mod route;

use menu::NavBar;
use route::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div class="container">
            <BrowserRouter>
                <NavBar/>
                <Switch<Route> render={ switch }/>
            </BrowserRouter>
        </div>
    }
}
