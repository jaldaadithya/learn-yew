use yew::{prelude::*, props};

#[function_component]
fn App(props : &Props) -> Html {
    let name = "Adithya Jalda".to_string();
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });
    let pre_made_props = props!(Props{});
    html! {
            <>
            <div>
                {"Hello World"} <img src="https://yew.rs/img/logo.svg" alt="yew logo"/> <p style="color:red">{name}</p>
                <Adithya ..pre_made_props/>
                <button {onclick}>{"button"}</button>
            </div>
            </>
    }
}

#[function_component]
fn Adithya(props : &Props) -> Html {
    return html! {<> {props.is_loading.clone()}  {props.name.clone()} </>};
}

fn main() {
    let cb = Callback::from(move |name:String| {
        format!("Calling from {}",name)
    });
    let emit = cb.emit("heyyyy".to_string());
    web_sys::console::log_1(&emit.into());
    yew::Renderer::<App>::new().render();
}

#[derive(Properties,PartialEq,Default)]
pub struct Props {
    #[prop_or(true)]
    pub is_loading : bool,
    #[prop_or(AttrValue::from("Adihya Jalda"))]
    pub name : AttrValue
}