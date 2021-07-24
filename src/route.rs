use yew_router::prelude::*;


#[derive(Switch, Debug, Clone)]
pub enum Route {
    // Note that order matters here, first match wins
    #[to = "/compiler/"]
    Compilerpage,
    #[to = "/"]
    Homepage,
}
