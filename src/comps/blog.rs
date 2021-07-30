use yew::prelude::*;

pub(crate) enum Msg {}

pub(crate) struct Blog {
    #[allow(dead_code)]
    link: ComponentLink<Self>,
}

impl Component for Blog {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="block content">
                <a href="https://gitlab.com/arxra/mysite/-/issues/2" class="box">
                    <span class="icon">
                        <i class="fa fa-gitlab"/>
                    </span>
                    {"To be implemented"}
                </a>
            </div>
        }
    }
}
