use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("bg-black", "flex", "flex-col", "items-center", "h-screen", "font-manrope")}>
            <div class={classes!("flex-grow", "flex", "flex-col", "items-center", "justify-center")}>
                <p class={classes!(
                    "font-extrabold", "text-transparent", "text-6xl", "lg:text-7xl",
                    "font-bold", "bg-gradient-to-r", "from-grad-1",
                    "via-grad-3", "to-grad-4", "bg-clip-text"
                )}>
                    {"Ali Alateyah"}
                </p>

                <p class={classes!("text-lg", "text-vista", "font-extralight", "lg:text-xl")}>
                    {"Backend Developer"}
                </p>
            </div>

            <div class={classes!(
                "flex", "flex-row", "gap-x-4", "mb-4", "text-sm", "lg:text-lg",
                "text-grey-cloud", "font-medium"
            )}>
                <a href="https://github.com/alidevs">{"GitHub"}</a>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
