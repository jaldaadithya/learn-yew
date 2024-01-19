use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let name = "Adithya Jalda".to_string();
    
    let html = html! {
            <>
            <div>
                {"Hello World"} <img src="https://yew.rs/img/logo.svg" alt="yew logo"/> <p style="color:red">{name}</p>
            </div>
            <div></div>
            </>
    };
    html
}

fn main() {
    yew::Renderer::<App>::new().render();
}
