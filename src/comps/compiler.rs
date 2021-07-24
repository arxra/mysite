use yew::{prelude::*, services::ConsoleService};
use cigrid::compile;

pub(crate) enum Msg {
    Compile,
    Update(String)
}

#[derive(Debug)]
pub(crate) struct CCompiler {
    link: ComponentLink<Self>,
    input: String,
    output: String,
    success: bool,
    loading: bool,
}

impl Component for CCompiler {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, input: String::new(),output: String::new(), success: true, loading: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Compile => {
                self.loading = true;
                println!("{}", self.input);
                match compile(self.input.clone()){
                    Ok(text) => {
                        ConsoleService::info(&format!("Success: {:?}", text));
                        self.output = text;
                        self.success = true
                    }
                    Err(failure) => {
                        self.output = failure.to_string();
                        if failure.code != 0 {
                            self.success = false;
                        }
                        ConsoleService::error(&format!("failed(code: {:?}): {:?}", &failure.code, &failure.message));
                    }
                }
                self.loading = false;
                true
            },
            Msg::Update(text) => {
                self.input = text;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut success_rend = vec!["textarea"];
        if self.success {success_rend.push("is-danger")} else {success_rend.push("is-info")}
        if self.loading {success_rend.push("is-loading")}
        let example = 
"#include <stdio.h>

extern int putchar(int x);

int main() {

  int x = 1;
  int y = 5;
  x = x + 4;
  x = x + y;
  putchar('A');
  putchar(x);
  return 0;
}
";

        html! {
            <div class="column">
                <div class="column is-one-third">
                    <textarea 
                        class="textarea" 
                        value=self.input.clone()
                        oninput=self.link.callback(|event: InputData| Msg::Update(event.value))
                        placeholder = example
                    ></textarea>
                </div>
                <div class="column">
                    <button class="button" onclick=self.link.callback(|_| Msg::Compile)>{"Compile"}</button>
                </div>
                <div class="column is-one-third">
                    <textarea
                        class=success_rend
                        readonly=true 
                        value=self.output.clone()
                    />
                </div>
            </div>
        }
    }
}
