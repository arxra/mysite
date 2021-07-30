use cigrid::compile;
use yew::{prelude::*, services::ConsoleService};
pub(crate) enum Msg {
    Compile,
    Default,
    Update(String),
}
#[derive(Debug)]
pub(crate) struct CCompiler {
    link: ComponentLink<Self>,
    input: String,
    output: String,
    success: bool,
    loading: bool,
    example: String,
}

impl Component for CCompiler {
    type Message = Msg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input: String::new(),
            output: String::new(),
            success: true,
            loading: false,
            example: "#include <stdio.h>

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
"
            .to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Compile => {
                self.loading = true;
                println!("{}", self.input);
                match compile(self.input.clone()) {
                    Ok(text) => {
                        ConsoleService::info(&format!("Success: {:?}", text));
                        self.output = text;
                        self.success = true
                    }
                    Err(failure) => {
                        if failure.code == 0 {
                            self.output = "No Input".to_string()
                        } else {
                            self.output = failure.to_string();
                        }
                        ConsoleService::error(&format!(
                            "failed(code: {:?}): {:?}",
                            &failure.code, &failure.message
                        ));
                    }
                }
                self.loading = false;
                true
            }
            Msg::Default => {
                self.input = self.example.clone();
                true
            }
            Msg::Update(text) => {
                self.input = text;
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut success_rend = vec!["textarea"];
        if self.success {
            success_rend.push("is-info")
        } else {
            success_rend.push("is-danger")
        }
        if self.loading {
            success_rend.push("is-loading")
        }
        html! {
            <>
                <div class="block content">
                    {"
                    A compiler for a subset of C, where the subset is quite small.
                    However, linking the output ASM will run and print the value of x and the charachter A.
                    "}
                </div>
                <div class="columns">
                    <div class="column">
                        <textarea
                            class="textarea"
                            value=self.input.clone()
                            oninput=self.link.callback(|event: InputData| Msg::Update(event.value))
                            placeholder = self.example.clone()
                            rows=30
                        ></textarea>
                    </div>
                    <div class="column is-2">
                        <div align="center">
                            <button class="button" onclick=self.link.callback(|_| Msg::Default)>{"Use C example"}</button>
                        </div>
                        <br/>
                        <div align="center">
                            <button class="button" onclick=self.link.callback(|_| Msg::Compile)>{"Compile"}</button>
                        </div>
                    </div>
                    <div class="column">
                        <textarea
                            class=success_rend
                            readonly=true
                            value=self.output.clone()
                            rows=30
                        />
                    </div>
                </div>
                <div class="block content">
                    {" While this "}
                        <a href="https://gitlab.com/arxra/my-cigrid-compiler">
                            <span class="icon">
                                <i class="fa fa-gitlab"/>
                            </span>
                            {"-project"}
                        </a>
                        {" is linked here, its not public by request of the examiner so access can be granted on request.
                        This compiler was written as part of ID2202 at Kungliga Tekniska Högskolan in Stockholm.
                        It just managed a passing grade, as there are many things missing but its a cool demonstration of Rust and Wasm.
                        I learned a lot during this project about Rust, so there _should_ be a blogpost on this site about it. Should..."}
                </div>
            </>
        }
    }
}
