use yew::prelude::*;

pub(crate) enum Msg {
    AddOne,
    SubOne,
}


pub(crate) struct UpDownButtons {
    link: ComponentLink<Self>,
    value: i32,
}

impl Component for UpDownButtons {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::SubOne => self.value -= 1
        }
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
                <button class="button" onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button class="button" onclick=self.link.callback(|_| Msg::SubOne)>{ "-1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}
