use yew::prelude::*; 
mod components;
static REPO: &str = "personal_website";
use yew_router::prelude::*;
use components::{
    about_me::AboutMe as AboutMe,
    contact::ContactMe as ContactMe,
    resume::Resume as Resume,
    blog::Blog as Blog,
    projects::Projects as Projects,
    papers::Papers as Papers,    
    markdown_component::MarkdownComponent as Markdown,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/personal_website/")]
    Home,
    #[at("/personal_website/about_me")]
    AboutMe,
    #[at("/personal_website/contact")]
    Contact,
    #[at("/personal_website/resume")]
    Resume,
    #[at("/personal_website/blog")]
    Blog,
    #[at("/personal_website/projects")]
    Projects,
    #[at("/personal_website/papers")]
    Papers,
    #[at("/personal_website/blogpost/:title")]
    Blogpost { title: String },
    #[at("/personal_website/projectpost/:title")]
    Projectpost { title: String },
    #[at("/personal_website/paperpost/:title")]
    Paperpost { title: String },
    #[not_found]
    #[at("/personal_website/404")]
    NotFound,
}





fn switch(routes: Route ) -> Html {
    let markdown = "# Hello, World!";
    
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
        Route::Papers => html! {
            <Papers />
        },
        Route::Blogpost { title } => html! {
            <p>{format!("You are looking at Blogpost {}", title)}</p>
        },
        Route::Projectpost { title } => html! {
            <p>{format!("You are looking at Projectpost {}", title)}</p>
        },
        Route::Paperpost { title } => html! {
            <p>{format!("You are looking at Paperpost {}", title)}</p>
        },



        //Route::Post => html! {
          //  <Markdown markdown={markdown}/>
        //},
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
                <a href="/personal_website/about_me" style="">{"Robert Junkins"}</a>
                <a href="/personal_website/contact">{"Contact"}</a>
                <a href="/personal_website/resume">{"Resume"}</a>
                <a href="/personal_website/blog">{"Blog"}</a>
                <a href="/personal_website/projects">{"Projects"}</a>
                <a href="/personal_website/papers">{"Papers"}</a>
            </nav>
            <BrowserRouter>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
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
