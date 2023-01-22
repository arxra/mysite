use cigrid::compile;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use log::info;
// use yew_hooks:

const EXAMPLE: &str = "
#include <stdio.h>

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

#[function_component(CCompiler)]
pub fn view() -> Html {
    let input_handler = use_state_eq(|| "".to_owned());
    let output = use_state_eq(|| "".to_owned());

    let oninput = {
        let input_handler = input_handler.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = input {
                input_handler.set(input.value());
            }
        })
    };
    let use_example = {
        let input_handler = input_handler.clone();
        Callback::from(move |_e: MouseEvent| {
            input_handler.set(EXAMPLE.to_owned());
        })
    };
    let click_compile = {
        let output = output.clone();
        let input = input_handler.clone();

        Callback::from(move |_e: MouseEvent| match compile(input.to_string()) {
            Ok(text) => output.set(text),
            Err(failure) => {
                if failure.code == 0 {
                    output.set("No Input".to_string())
                } else {
                    output.set(failure.to_string())
                }
            }
        })
    };
    html! {
        <>
            <div class="block content">
                {"
                    A compiler for a subset of C, where the subset is quite small.
                    However, linking the output x86-ASM will run and print the value of x and the character A.
                "}
            </div>
            <div class="columns is-vcentered">
                <div class="column">
                    <textarea
                        class="textarea"
                        {oninput}
                        value={ input_handler.to_string() }
                        placeholder = 	{ EXAMPLE }
                        rows=30
                    />
                </div>
                <div class="column is-2">
                    <div align="center">
                        <button class="button" onclick={use_example}>{"Use C example"}</button>
                    </div>
                    <br/>
                    <div align="center">
                        <button class="button" onclick={click_compile}>{"Compile"}</button>
                    </div>
                </div>
                <div class="column">
                    <textarea
                        class="textarea"
                        readonly=true
                        value={output.to_string()}
                        rows=30
                    />
                </div>
            </div>
            <div class="block content">
                {" Source: "}
                    <a href="https://gitlab.com/arxra/my-cigrid-compiler">
                        <span class="icon">
                            <i class="fa fa-gitlab"/>
                        </span>
                    </a>
                    {".
                        This compiler was written as part of ID2202 at Kungliga Tekniska Högskolan in Stockholm.
                        It just managed a passing grade, as there are many things missing but its a cool demonstration of Rust and Wasm.
                        Yes, I would have liked to earn a higher grade, but it turns out learning rust and how to write a compiler in the same 10 weeks is really hard.
                        Not to mention, that was just one of the courses in that period... Yes I'm making excuses here.
                        I learned a lot during this project about Rust and compilers, so there _should_ be a blogpost on this site about it.
                        Should.
                        In the meantime, see the readme of the compiler."
                    }
            </div>
        </>
    }
}
