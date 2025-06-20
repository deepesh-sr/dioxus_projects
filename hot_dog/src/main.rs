use dioxus::{html::link::title, prelude::*};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { div {
        id : "title",
        h1 { "Hotdog !" }
     }
    div {
        id : "dogview",
        img { src: "https://i.pinimg.com/474x/f0/69/3e/f0693eab182b9c490eed90b96f788723.jpg" }
     }
    div {id : "buttons",
     button { id:"skip" ,"Skip" }
     button { id:"save","Save"  }
    }}
}

#[derive(Props, PartialEq, Clone)]
struct HotDogProps {
    breed: String,
}

#[component]
fn DogApp(breed: String) -> Element {
    todo!()
}
