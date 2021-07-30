use yew::prelude::*;

pub(crate) enum Msg {}

pub(crate) struct Home {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="block content">
                    <span class="title is-1" >{"Welcome to my page!"}</span>
                </div>
                <div class="block">
                    <p>
                        {"I am not the good with words but there is a subset of c compiler here written in rust and brought to your browser through "}
                        <a href="https://yew.rs">{"the yew WASM framwork"}</a>
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
}
