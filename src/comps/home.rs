use yew::prelude::*;

#[function_component]
pub fn HomePage() -> Html {
    html! {
        <>
            <div class="block content">
                <span class="title is-1" >{"Welcome to my page!"}</span>
            </div>
            <div class="block">
                <p>
                    {"I am not the good with words but there is a subset of c compiler here written in rust and brought to your browser through "}
                    <a href="https://yew.rs">{"the yew frontend framwork"}</a>
                    {" which compiles everything down to WASM. Neat!"}
                </p>
            </div>
            <div class="block">
                <p>
                    {"Other noteworthy tech here are "}
                    <a href="https://bulma.io">{"bulma css"}</a>
                    {" and "}
                    <a href="https://fontawesome.com/v4.7/icons/">{"font awsome"}</a>
                    {" which display the pretty gitlab logos. All of those logos link to different things for the "}
                    <a href="https://gitlab.com/arxra/mysite">
                        <span class="icon">
                            <i class="fa fa-gitlab"/>
                        </span>
                    </a>
                    {"-project for this here site source code."}
                </p>
            </div>
        </>
    }
}
