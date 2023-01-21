use yew::prelude::*;
use yew_router::prelude::*;

use crate::comps::blog::BlogPage;
use crate::comps::compiler::CCompiler;
use crate::comps::home::HomePage;

#[derive(Routable, Debug, Clone, PartialEq, Copy)]
pub enum Route {
    #[at("/")]
    Homepage,
    #[at("/blog/")]
    Blogpage,
    #[at("/compiler/")]
    Compilerpage,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Homepage => html! {<HomePage/>},
        Route::Blogpage => html! {<BlogPage/>},
        Route::Compilerpage => html! {<CCompiler/>},
    }
}
