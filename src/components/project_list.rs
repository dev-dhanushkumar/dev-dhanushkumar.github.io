use leptos::prelude::*;
use crate::pages::projects::projects::Project;


#[component]
pub fn ProjectList(projects: Vec<Project>) -> impl IntoView {
    view! {
        <div class="grid grid-cols-12 gap-5">
            {projects.into_iter().map(|project| {
                view! {
                    <div class="col-span-12 sm:col-span-6 lg:col-span-4 border dark:border-zinc-700 rounded-xl shadow-sm p-5">
                        // tags
                        <div class="flex justify-between items-center">
                            <div class="flex gap-2">
                                {project.tags.unwrap_or_default().into_iter().map(|tag| {
                                    view! {
                                        <span class="border border-zinc-300 dark:border-zinc-700 rounded-2xl text-sm text-zinc-700 dark:text-zinc-300 no-underline px-2 py-0.5 transition-all duration-700 hover:border-zinc-700 dark:hover:border-zinc-300">
                                            {tag}
                                        </span>
                                    }
                                }).collect_view()}
                            </div>

                            // stargazers
                            // {project.stargazers_count.map(|stars| {
                            //     view! {
                            //         <div class="shrink-0">
                            //             <a href={project.html_url.clone().unwrap_or_default()} target="_blank" class="...">
                            //                 <img src="/github-mark.svg" class="dark:hidden" height="12" alt="GitHub stars" />
                            //                 <img src="/github-mark-white.svg" class="hidden dark:block" height="12" alt="GitHub stars" />
                            //                 {format!("{} stars", stars)}
                            //             </a>
                            //         </div>
                            //     }
                            // })}
                        </div>

                        // name
                        <h3 class="font-semibold my-2">
                            <a class="underline" href={project.demo_link.clone()} target="_blank" rel={project.demo_link.clone()}>
                                {project.name}
                            </a>
                        </h3>

                        // description
                        <div class="line-clamp-2 text-zinc-600 dark:text-zinc-300 mb-4 min-h-[50px]">
                            <p>{project.description.clone()}</p>
                        </div>

                        // links
                        <div class="flex justify-end gap-3">
                            {project.post_link.clone().map(|link| {
                                view! {
                                    <a class="underline flex items-center gap-2" href={link}>
                                        "Article"
                                    </a>
                                }
                            })}
                            <a class="underline flex items-center gap-2" href={project.demo_link.clone()} target="_blank" rel={project.demo_link.clone()}>
                                "Link"
                                <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-external-link"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/></svg>
                            </a>
                        </div>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}
