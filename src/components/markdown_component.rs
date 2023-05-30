use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

pub fn markdown_to_html(markdown: &String) -> String{
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    String::from(html_output)

}

#[function_component(MarkdownComponent)]
pub fn markdown_component(props: &Props) -> Html {
    let raw_html = markdown_to_html(&props.markdown.clone());
    let div = gloo_utils::document().create_element("div").unwrap();

    div.set_inner_html(&raw_html.clone());

    Html::VRef(div.into())
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub markdown: String,
}

