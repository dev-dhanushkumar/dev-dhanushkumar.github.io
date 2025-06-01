use leptos::prelude::*;
use crate::{content::blog::load_blog_posts, utils::{slugify}};
use std::collections::HashMap;

#[component]
pub fn Tags() -> impl IntoView {
    let posts = load_blog_posts();
    let mut tag_map: HashMap<String, (String, usize)> = HashMap::new();

    for post in &posts {
        if let Some(tags) = &post.tags {
            for tag in tags {
                let slug = slugify(tag);
                let entry = tag_map.entry(slug.clone()).or_insert((tag.clone(), 0));
                entry.1 += 1;
            }
        }
    }

    let mut grouped: HashMap<char, Vec<(String, String, usize)>> = HashMap::new();
    for (slug, (label, count)) in tag_map {
        let first_char = label.chars().next().unwrap_or('#').to_ascii_uppercase();
        grouped.entry(first_char).or_default().push((slug, label, count));
    }

    let mut grouped_vec: Vec<(char, Vec<(String, String, usize)>)> = grouped.into_iter().collect();
    grouped_vec.sort_by_key(|(ch, _)| *ch);
    for (_, tags) in &mut grouped_vec {
        tags.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
    }

    view! {
        <div class="@container mx-auto flex flex-col mt-10 justify-between w-full max-w-5xl px-4 sm:px-6 lg:px-8">
            <div class="mb-10">
                <h1 class="text-3xl font-bold mb-2">"All Tags"</h1>
                <p class="text-zinc-700 dark:text-zinc-300 text-lg">"Post tags: concise keywords categorizing content for easy navigation and improved searchability"</p>
            </div>
            {
                grouped_vec.into_iter().map(|(letter, tags)| {
                    view! {
                        <div class="flex gap-2 border-b dark:border-b-zinc-700 py-3 mb-5 capitalize">
                            <h2 class="text-2xl font-bold text-zinc-800 dark:text-zinc-200 w-7 shrink-0">{letter.to_string()}</h2>
                            <div class="flex gap-2 flex-wrap">
                                {
                                    tags.into_iter().map(|(slug, label, count)| {
                                        view! {
                                            <a
                                                class="border border-zinc-300 dark:border-zinc-700 rounded-2xl
                                                    text-sm text-zinc-700 dark:text-zinc-300 no-underline px-3 py-1
                                                    transition-all duration-700
                                                    hover:border-zinc-700 dark:hover:border-zinc-300"
                                                href=format!("/tags/{}/", slug)
                                            >
                                                {format!("{} ({})", label, count)}
                                            </a>
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}
