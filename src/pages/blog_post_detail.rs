use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::{components::blog_post_detail_view::BlogPostDetailView, content::blog::get_blog_post_by_slug};

use super::not_found::NotFound;




#[component]
pub fn BlogPostDetails() -> impl IntoView {
    let params = use_params_map();
    let bindings = params.get().get("posts");
    let posts_slug = bindings.as_deref().unwrap_or_default();

    let get_posts = get_blog_post_by_slug(posts_slug);
    let content: AnyView = match get_posts {
        Some(post) => {
            let title = post.title;
            let description = post.description;
            let pub_date = post.pub_date;
            let updated_date = post.updated_date;
            let tags = post.tags.unwrap_or_default();
            let cover_image_url = post.cover_image;
            
            view! {
                <BlogPostDetailView
                    title=title
                    _description= description
                    pub_date=pub_date
                    updated_date=updated_date
                    tags=tags
                    cover_image_url=cover_image_url
                    slug = posts_slug.to_string()
                />
            }
            .into_any()
        }
        None => view! { <NotFound /> }.into_any(), // Handle the case where the post is not found
    };

    content
}
