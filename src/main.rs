use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="p-8">
            <p class="font-medium text-red-500">{ "Hello from Yew!" }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
