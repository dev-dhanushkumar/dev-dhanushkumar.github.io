use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, Event, Tag};
use crate::utils::slugify;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Heading {
    pub depth: u8,
    pub slug: String,
    pub text: String,
}



pub fn _extract_headings(markdown: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let parser = Parser::new(markdown);

    let mut in_heading = false;
    let mut current_text = String::new();
    let mut current_level = 0;

    for event in parser {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                in_heading = true;
                current_level = level as u8;
                current_text.clear();
            }
            Event::End(Tag::Heading(_, _, _)) => {
                if in_heading && current_level >= 2 && current_level <= 3 {
                    let slug = slugify(&current_text);
                    headings.push(Heading {
                        depth: current_level,
                        slug,
                        text: current_text.clone(),
                    });
                }
                in_heading = false;
            }
            Event::Text(text) | Event::Code(text) => {
                if in_heading {
                    current_text.push_str(&text);
                }
            }
            _ => {}
        }
    }

    headings
}









#[component]
pub fn TableOfContent(headings: Vec<Heading>, max_level: Option<u8>) -> impl IntoView {
    let max_level = max_level.unwrap_or(3);

    let toc = {
        let mut result = vec![];
        let mut parents = std::collections::HashMap::new();

        for heading in &headings {
            let item = heading.clone();
            parents.insert(item.depth, item.clone());

            if item.depth == 2 {
                result.push((item, vec![]));
            } else if max_level > 2 {
                if let Some(parent) = parents.get_mut(&(item.depth - 1)) {
                    result
                        .iter_mut()
                        .rev()
                        .find(|(h, _)| h.slug == parent.slug)
                        .map(|(_, subs)| subs.push(item.clone()));
                }
            }
        }
        result
    };

    view! {
        <nav class="toc sticky top-4 py-2 lg:-ml-3">
            <h3 class="font-extrabold uppercase mb-1 px-3 text-sm">"On This Page"</h3>
            <ul class="toc-list max-h-[calc(100vh-70px)] overflow-auto text-[.825rem] text-[var(--color-content-secondary)]">
                {toc.into_iter().map(|(heading, subheadings)| view! {
                    <li>
                        <a
                            class="block font-bold py-1.5 px-3 border-l border-transparent text-inherit leading-tight text-zinc-700 dark:text-zinc-300"
                            href={format!("#{}", heading.slug)}
                        >
                            {heading.text}
                        </a>
                        {
                            let subheadings_for_when = subheadings.clone();
                            let subheadings_for_list = subheadings.clone();
                            view! {
                                <Show
                                    when=move || !subheadings_for_when.is_empty()
                                    fallback=|| view! { <ul></ul> }
                                >
                                    <ul>
                                        { subheadings_for_list.iter().cloned().map(|subheading| view! {
                                            <li>
                                            <a
                                                class="block py-1.5 pl-6 pr-3 border-l border-transparent text-inherit leading-tight text-zinc-700 dark:text-zinc-300"
                                                href={format!("#{}", subheading.slug)}
                                            >
                                                {subheading.text}
                                            </a>
                                            </li>
                                        }).collect_view()
                                        }
                                    </ul>
                                </Show>
                            }
                        }
                    </li>
                }).collect_view()}
            </ul>
        </nav>
    }
}
