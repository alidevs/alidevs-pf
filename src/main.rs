use chrono::{Local, TimeZone};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,

    #[at("/oct")]
    October,
}

#[function_component(Home)]
fn home() -> Html {
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

#[function_component(Oct)]
fn october() -> Html {
    let now = Local::now();
    let oct = Local.with_ymd_and_hms(2024, 10, 1, 0, 0, 0).unwrap();
    let days_left = oct.signed_duration_since(now).num_days();

    html! {
        <div class={classes!("bg-black", "flex", "flex-col", "items-center", "h-screen", "font-manrope")}>
            <div class={classes!("flex-grow", "flex", "flex-col", "items-center", "justify-center")}>
                <p class={classes!(
                    "font-extrabold", "text-transparent", "text-6xl", "lg:text-7xl",
                    "font-bold", "bg-gradient-to-r", "from-grad-1",
                    "via-grad-3", "to-grad-4", "bg-clip-text"
                )}>
                    {days_left}
                </p>

                <p class={classes!("text-lg", "text-vista", "font-extralight", "lg:text-xl")}>
                    {"Days until October"}
                </p>
            </div>

            <div class={classes!(
                "mb-4", "text-sm", "lg:text-lg",
                "text-grey-cloud", "font-light"
            )}>
                <p>{format!("{}", now.format("%A, %B %d, %Y"))}</p>
            </div>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::October => html! { <Oct /> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
