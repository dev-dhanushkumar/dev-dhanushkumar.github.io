use leptos::prelude::*;
use crate::{content::blog::BlogPost, components::post_item::PostItem};
use std::collections::HashMap;
use chrono::Datelike;


#[component]
pub fn PostsByYear(posts: Vec<BlogPost>) -> impl IntoView {
    // 1) group posts by year
    let mut map: HashMap<i32, Vec<BlogPost>> = HashMap::new();
    for post in posts {
        map.entry(post.pub_date.year())
            .or_default()
            .push(post);
    }
    // 2) collect into a Vec and sort years descending
    let mut groups: Vec<(i32, Vec<BlogPost>)> = map.into_iter().collect();
    groups.sort_by(|a, b| b.0.cmp(&a.0));
    // 3) sort each year's posts by pub_date descending
    for (_, posts) in groups.iter_mut() {
        posts.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
    }


    view! {
        <div>
            { groups.into_iter().map(|(year, posts)| {
                view! {
                    <div class="mb-8">
                        <h2 class="text-2xl font-bold mb-4">
                            { move || year.to_string() }
                        </h2>
                        <div>
                            { posts.into_iter().map(|post| {
                                view! {
                                    <PostItem post=post />
                                }
                            }).collect_view() }
                        </div>
                    </div>
                }
            }).collect_view() }
        </div>
    }
}