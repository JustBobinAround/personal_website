use yew::prelude::*; 
mod components;

static REPO: &str = "personal_website";
use yew_router::prelude::*;
use components::{
    about_me::AboutMe as AboutMe,
    contact::ContactMe as ContactMe,
    resume::Resume as Resume,
    bloglist::Bloglist as Bloglist,
    projects::Projects as Projects,
    papers::Papers as Papers,    
    markdown_component::MarkdownComponent as Markdown,
    blogpost::Blogpost as Blogpost,
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
    Bloglist,
    #[at("/projects")]
    Projects,
    #[at("/papers")]
    Papers,
    #[at("/blogposts/:title")]
    Blogpost { title: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}





fn switch(routes: Route ) -> Html {
    
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
        Route::Bloglist => html! { 
            <Bloglist />
        },
        Route::Projects => html! { 
            <Projects />
        },
        Route::Papers => html! {
            <Papers />
        },
        Route::Blogpost { title } => html! {
            <Blogpost title={title}/>
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
                <a href="#/about_me" style="">{"Robert Junkins"}</a>
                <a href="#/contact">{"Contact"}</a>
                <a href="#/resume">{"Resume"}</a>
                <a href="#/blog">{"Blog"}</a>
                <a href="#/projects">{"Projects"}</a>
                <a href="#/papers">{"Papers"}</a>
            </nav>
            <HashRouter basename="/personal_website">
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </HashRouter>
            <br/>
            <hr/>
            <pre >{"
 * Copyright 2022-2023 Robert Junkins. 
"}</pre>
        </div>

    }
}

fn main() {
     
   yew::Renderer::<App>::new().render();
}
