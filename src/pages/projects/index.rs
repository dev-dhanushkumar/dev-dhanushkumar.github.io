use leptos::prelude::*;
use crate::{components::project_list::ProjectList, pages::projects::projects::load_projects};

#[component]
pub fn Projects() -> impl IntoView {
    let projects = load_projects().into_iter().collect::<Vec<_>>();
    view! {
        <div class="@container mx-auto flex flex-col mt-10 justify-between w-full max-w-5xl px-4 sm:px-6 lg:px-8">
            <div class="mb-10">
            <h1 class="text-3xl font-bold mb-2">"All My Projects"</h1>
            <p class="text-zinc-700 dark:text-zinc-300 text-lg">"All my project portfolio from real projects to open source projects."</p>
            </div>
            <ProjectList projects={projects}/>
        </div>
    }
}