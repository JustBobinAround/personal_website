use yew::prelude::*;
use web_sys::{window};
use pulldown_cmark::{html, Options, Parser};

#[derive(Properties, PartialEq)]
struct CallbackHelperProps {
    pub helper: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct UrlProps {
    pub url: String,
}

#[function_component(CallbackHelper)]
fn callback_helper(props: &CallbackHelperProps) -> Html {
    props.helper.emit(String::from(""));
    html! {
        <></>
    }
}
#[function_component(MarkdownComponent)]
pub fn markdown_component(props: &UrlProps) -> Html {
    let inner = use_state(|| "Loading markdown file...".to_owned());
    let url = process_request(&props.url.clone());
    let get_markdown = Callback::from( {
        let inner = inner.clone();
        let url = url.clone();
        move |_| {
            wasm_bindgen_futures::spawn_local({
                let inner = inner.clone();
                let url = url.clone();
                async move {
                    let content = reqwest::get(url)
                        .await.unwrap()
                        .text()
                        .await.unwrap();
                    let content = markdown_to_html(&content);
                    inner.set(content);
                }
            });
        }
    });
    let test = Html::from_html_unchecked((*inner).clone().into());
    html! {
        <div>
            <CallbackHelper helper={get_markdown} />
            {test}
        </div>
    }
}


fn markdown_to_html(markdown: &String) -> String{
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    String::from(html_output)

}

fn process_request(filename: &str) -> String{
    let window = window().unwrap();

    // Get the root URL of the website
    let root_url = window.location().origin().unwrap();
    let full_url = format!("{}{}", root_url, filename);
    
    return full_url;
}

