use log::info;
use pulldown_cmark::{Event, Options, Parser, Tag};
use regex::Regex;
use crate::components::table_of_content::Heading;


// pub fn markdown_to_html_view(markdown: &str) -> (impl IntoView, Vec<Heading> ){

//     // info!("Converting markdown file at path: {}", &path);
//     // // /blog/2025-03-08-basic-of-webgpu-part1/index.md
//     // let markdown = fs::read_to_string(&path)
//     //     .unwrap_or_else(|e| panic!("Failed to read markdown file at path: {:?}", e));

//     info!("markdown data: {:?}", markdown);

//     let mut headings = Vec::new();
//     let mut events = Vec::new();

//     let options = Options::all();
//     let parser = Parser::new_ext(&markdown, options);

//     let heading_slugify = Regex::new(r"[^\w]+").unwrap();

//     let mut in_heading = false;
//     let mut current_heading = (0, String::new());

//     for event in parser {
//         match &event {
//             Event::Start(Tag::Heading(level, _, _)) => {
//                 in_heading = true;
//                 current_heading.0 = *level as u8;
//                 current_heading.1.clear();
//                 events.push(event.clone());
//             }
//             Event::End(Tag::Heading(_, _, _)) => {
//                 if in_heading {
//                     in_heading = false;
//                     let slug = heading_slugify
//                         .replace_all(&current_heading.1.to_lowercase(), "-")
//                         .trim_matches('-')
//                         .to_string();

//                     headings.push(Heading {
//                         depth: current_heading.0,
//                         slug: slug.clone(),
//                         text: current_heading.1.clone(),
//                     });

//                     // Inject anchor
//                     events.push(Event::Html(format!(r#"<a id="{slug}" class="anchor"></a>"#).into()));
//                 }
//                 events.push(event.clone());
//             }
//             Event::Text(text) => {
//                 if in_heading {
//                     current_heading.1.push_str(&text);
//                 }
//                 events.push(event.clone());
//             }
//             _ => {
//                 events.push(event.clone());
//             }
//         }
//     }

//     let mut html_output = String::new();
//     html::push_html(&mut html_output, events.into_iter());

//     let view = view! {
//         <div class="prose dark:prose-invert max-w-none" inner_html=html_output></div>
//     };
    

//     (view, headings)
// }




pub fn markdown_to_html_view(
    markdown: String,
) -> Vec<Heading> {
    // 1) parse headings + build owned HTML
    info!("markdown data: {:?}", markdown);
    let mut headings = Vec::new();
    let mut events  = Vec::new();
    let mut in_h    = false;
    let mut cur     = (0u8, String::new());
    let slugify     = Regex::new(r"[^\w]+").unwrap();

    for ev in Parser::new_ext(&markdown, Options::all()) {
        match &ev {
            Event::Start(Tag::Heading(lvl, _, _)) => {
                in_h = true;
                cur.0 = *lvl as u8;
                cur.1.clear();
                events.push(ev.clone());
            }
            Event::End(Tag::Heading(_, _, _)) => {
                in_h = false;
                let slug = slugify
                    .replace_all(&cur.1.to_lowercase(), "-")
                    .trim_matches('-')
                    .to_string();
                headings.push(Heading { depth: cur.0, slug: slug.clone(), text: cur.1.clone() });
                events.push(Event::Html(format!(r#"<a id="{slug}"></a>"#).into()));
                events.push(ev.clone());
            }
            Event::Text(t) if in_h => {
                cur.1.push_str(t);
                events.push(ev.clone());
            }
            other => events.push(other.clone()),
        }
    }
    // let mut html_output = String::new();
    // html::push_html(&mut html_output, events.into_iter());

    // // 2) build a 'static view that *moves* html_output into its closure
    // let view = view! {
    //     <div
    //       class=""
    //       inner_html=move || html_output.clone()
    //     />
    // };

    headings
}
