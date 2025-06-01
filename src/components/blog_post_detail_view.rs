use leptos::prelude::*;
use log::info;
use pulldown_cmark::{html, Options, Parser};
use crate::{components::{about_the_author::AboutTheAuthor, markdown_to_html_view::markdown_to_html_view, prose::Prose, table_of_content::TableOfContent}, content::blog::strip_front_matter, utils::slugify};
use chrono::NaiveDate;

use include_dir::{include_dir, Dir};
static BLOG_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/content/blog");

#[component]
pub fn BlogPostDetailView(
    title: String,
    _description: String,
    cover_image_url: Option<String>,
    pub_date: NaiveDate,
    updated_date: Option<NaiveDate>,
    tags: Vec<String>,
    slug: String,
) -> impl IntoView {
    let update_date = updated_date.clone().map(|date| date.format("%b %d, %Y").to_string()).unwrap();
    info!("get slug details: {:?}", slug);
    let blog_path = format!("{slug}/index.md");
    let md = BLOG_DIR.get_file(&blog_path).unwrap().contents_utf8().unwrap();
    let body = strip_front_matter(&md).to_string();
    let body1 = body.clone();
    let blog = markdown_to_html_view(body);
    let cover_image_url = cover_image_url.unwrap();
    info!("cover image URL: {:?}", cover_image_url);


    // let headings: Vec<Heading> = extract_headings(&markdown);

    view! {
            <div class="@container mx-auto flex mt-10 justify-between w-full max-w-6xl px-4 sm:px-6 lg:px-8 gap-8">
                <main class="overflow-hidden grow">
                    <article>
                        <Prose>
                            <div>
                                <h1 class="!my-1 text-4xl leading-none"><strong>{title.clone()}</strong></h1>
                                <div class="text-sm font-[500] mt-2 sm:mt-0 py-1 md:text-right flex flex-col sm:flex-row gap-3 sm:justify-between sm:items-center">
                                    <div class="flex gap-2">
                                        {tags.iter().map(|tag| view! {
                                            <a
                                                class="border border-zinc-300 dark:border-zinc-700 rounded-2xl
                                                text-sm text-zinc-700 dark:text-zinc-300 no-underline px-2 py-0.5
                                                transition-all duration-700
                                                hover:border-zinc-700 dark:hover:border-zinc-300"
                                                href={format!("/tags/{}", slugify(tag))}
                                            >
                                                {tag.clone()}
                                            </a>
                                        }).collect_view()}
                                    </div>
                                    <div>
                                        {updated_date.clone().map(|_| view! {
                                            <>"Updated on "<strong>{pub_date.format("%b %d, %Y").to_string()}</strong></>
                                        }).unwrap_or_else(|| view! {
                                            <>"Published on "<strong>{update_date}</strong></>
                                        })}
                                    </div>
                                </div>
                            </div>
                            <div class="py-3 overflow-hidden">
                                <img class="rounded-3xl w-full m-0 lg:mb-2" src={cover_image_url} alt=title.clone() loading="eager" />
                            </div>
                            // {blog.0}
                            {
                                let options = Options::all();
                                let parser = Parser::new_ext(&body1, options);
                                let mut html_output = String::new();
                                html::push_html(&mut html_output, parser);
                                // view! {
                                //     <div class=""
                                // }
                            }
                        </Prose>
                    </article>
                </main>
                <div class="shrink-0 w-[280px] hidden md:block">
                    <div class="mb-4">
                        <AboutTheAuthor />
                    </div>
                    <TableOfContent headings=blog max_level=Some(3) />
                </div>
            </div>
    }
}  