use leptos::prelude::*;
use crate::{content::blog::load_blog_posts, components::post_by_year::PostsByYear};

#[component]
pub fn Posts() -> impl IntoView {
    let posts = load_blog_posts().into_iter().collect::<Vec<_>>();
    view! {
        <div class="@container mx-auto flex flex-col mt-10 justify-between w-full max-w-5xl px-4 sm:px-6 lg:px-8">
            <div class="mb-10">
            <h1 class="text-3xl font-bold mb-2">"All Posts"</h1>
            <p class="text-zinc-700 dark:text-zinc-300 text-lg">"Hands-on guides, code walkthroughs, and technical articles covering technologies."</p>
            </div>
            <PostsByYear posts={posts}/>
        </div>
    }
}