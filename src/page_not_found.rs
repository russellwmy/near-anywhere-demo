use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component]
pub fn PageNotFound(_: &Props) -> Html {
    html! {
        <div class="h-[calc(100vh-60px)] w-full flex items-center justify-center">
            <h1
                class="text-stone-700 text-5xl font-bold"
            >
                {"Page not found"}
            </h1>
        </div>
    }
}
