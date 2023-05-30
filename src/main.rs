use yew::prelude::*; 
mod components;

use yew_router::prelude::*;
use components::{
    about_me::AboutMe as AboutMe,
    contact::ContactMe as ContactMe,
    resume::Resume as Resume,
    blog::Blog as Blog,
    projects::Projects as Projects,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/about_me")]
    AboutMe,
    #[at("/contact")]
    Contact,
    #[at("/resume")]
    Resume,
    #[at("/blog")]
    Blog,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}






fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <AboutMe />
        },
        Route::AboutMe => html! { 
            <AboutMe />
        },
        Route::Contact => html! { 
            <ContactMe />
        },
        Route::Resume => html! { 
            <Resume />
        },
        Route::Blog => html! { 
            <Blog />
        },
        Route::Projects => html! { 
            <Projects />
        },
        Route::NotFound => html! {
            <div class="reader-mode">
                <h1>{ "404" }</h1> 
                <p>{"Page doesn't exist. How did you get here?"}</p>
            </div> 
        },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="reader-mode">

            <nav>               
                <a href="/about_me" style="">{"Robert Junkins"}</a>
                <a href="/contact">{"Contact"}</a>
                <a href="/resume">{"Resume"}</a>
                <a href="/blog">{"Blog"}</a>
                <a href="/projects">{"Projects"}</a>
            </nav>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
            <br/>
            <hr/>
            <pre >{"
 * Copyright 2022-2023 Robert Junkins. 
 * All opinions within this web application do not represent 
 * any of my employer; Future, Past, or Present
"}</pre>
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
