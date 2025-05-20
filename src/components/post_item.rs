use leptos::prelude::*;
use crate::content::blog::BlogPost;
use crate::utils::slugify;
// use chrono::Utc;

#[component]
pub fn PostItem(post: BlogPost) -> impl IntoView {
    let date = post.pub_date.format("%b %d, %Y").to_string();
    let title = post.title.clone();
    let post_url = format!("/posts/{}", post.slug);

    view! {
        <div class="flex flex-col sm:flex-row gap-2 sm:items-center border-b py-2 mb-1 capitalize dark:border-b-zinc-700">
            <div class="text-zinc-700 text-sm w-24 dark:text-zinc-300 shrink-0">
                // <time datetime={post.pub_date.to_rfc3339()}>{date}</time>
                <time datetime={post.pub_date.format("%Y-%m-%d").to_string()}>{date}</time>
            </div>
            <h3 class="font-medium grow">
                // <a href={format!("/posts/{}", post.slug)}>{&post.title}</a>
                <a href={post_url}>{ move || title.clone() }</a>
            </h3>
            <div class="flex gap-2 shrink-0">
                {
                    post.tags.clone().unwrap_or_default().into_iter().map(|tag| {
                        let slug = slugify(&tag);
                        view! {
                            <a class="border border-zinc-300 dark:border-zinc-700 rounded-2xl text-sm text-zinc-700 dark:text-zinc-300 no-underline px-2 py-0.5 transition-all duration-700 hover:border-zinc-700 dark:hover:border-zinc-300"
                               href={format!("/tags/{}", slug)}>
                                {tag}
                            </a>
                        }
                    }).collect_view()
                }
            </div>
        </div>
    }
}
