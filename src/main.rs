use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("bg-black", "flex", "flex-col", "items-center", "h-screen", "font-manrope")}>
            <div class={classes!("flex-grow", "flex", "flex-col", "items-center", "justify-center")}>
                <p class={classes!(
                    "my-2", "font-extrabold", "text-transparent", "text-6xl",
                    "font-bold", "bg-gradient-to-r", "from-grad-1",
                    "via-grad-3", "to-grad-4", "bg-clip-text"
                )}>
                    {"Ali Alateyah"}
                </p>

                <p class={classes!("text-lg", "text-vista", "font-light")}>
                    {"Backend Developer"}
                </p>
            </div>

            <div class={classes!(
                "flex", "flex-row", "gap-x-4", "mb-4", "text-sm",
                "text-grey-cloud", "font-light"
            )}>
                <a href="https://github.com/alidevs">{"GitHub"}</a>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
