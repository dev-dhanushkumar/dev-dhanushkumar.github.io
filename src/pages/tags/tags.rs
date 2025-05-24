use leptos::prelude::*;
use leptos_router::hooks::use_params;
use crate::{content::blog::load_blog_posts, components::post_by_year::PostsByYear, utils::{slugify, unslugify}};

#[component]
pub fn TagPage() -> impl IntoView {
    // Tell use_params we want a HashMap<String,String>
    let params = use_params::<std::collections::HashMap<String, String>>();
    // now pull out the "tag" key
    let tag = params
        .get()
        .get("tag")
        .cloned()
        .unwrap_or_default();

    let all_posts = load_blog_posts();
    let tagged_posts = all_posts
        .into_iter()
        .filter(|post| {
            post.tags
                .as_ref()
                .map(|tags| tags.iter().any(|t| slugify(t) == tag))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();

    let title = format!("All Posts Tagged with {}", unslugify(&tag));

    view! {
        <div class="container mx-auto px-4 sm:px-6 lg:px-8 mt-10">
          <div class="mb-10">
            <h1 class="text-3xl font-bold mb-2">{title}</h1>
          </div>
          <PostsByYear posts={tagged_posts}/>
        </div>
    }
}