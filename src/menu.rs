use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <div class="navbar is-info has-shadow" role="navigation">
            <div class="navbar-menu is-active">
                <div class="navbar-start">
                    <Link<Route> to={Route::Homepage} classes="navbar-item">
                        {"Home"}
                    </Link<Route>>
                    <Link<Route> to={ Route::Blogpage } classes="navbar-item">
                        {"Blog Posts"}
                    </Link<Route>>
                    <Link<Route> to={ Route::Compilerpage } classes="navbar-item">
                        {"Toy Compiler"}
                    </Link<Route>>
                </div>
                <div class="navbar-end">
                    <a href="https://gitlab.com/arxra/mysite" class="navbar-item">
                        <span class="icon">
                            <i class="fa fa-gitlab"/>
                        </span>
                    </a>
                </div>
            </div>
        </div>
    }
}
