use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

#[derive(Debug)]
pub(crate) struct Navbar {
    link: ComponentLink<Self>,
}

impl Component for Navbar {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
        <div class="navbar" role="navigation">
            <div class="navbar-brand">
            </div>
            <div class="navbar-menu">
                <div class="navbar-start">
                    <Anchor route=Route::Homepage classes="navbar-item">
                    {"Home"}
                    </Anchor>
                    <Anchor route=Route::Compilerpage classes="navbar-item">
                    {"Toy Compiler"}
                    </Anchor>
                </div>
            </div>
        </div>
        }
    }
}
