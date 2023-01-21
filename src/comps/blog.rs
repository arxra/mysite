use yew::prelude::*;

#[function_component(BlogPage)]
pub fn blog_page() -> Html {
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
