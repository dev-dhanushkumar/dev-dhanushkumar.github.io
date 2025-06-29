use leptos::prelude::*;

#[component]
pub fn Prose(children: Children) -> impl IntoView {
    view! { 
        <div
            class="prose dark:prose-invert
                prose-h1:font-[900] prose-h1:my-4 prose-h1:text-2xl sm:prose-h1:text-[2rem]/[1.3]
                prose-h2:mt-6 prose-h2:mb-3 prose-h2:text-2xl sm:prose-h2:text-3xl prose-h2:font-extrabold
                prose-h3:mt-5 prose-h3:mb-2 prose-h3:text-xl sm:prose-h3:text-2xl
                prose-h4:mt-3 prose-h4:mb-0 prose-img:rounded-xl prose-h3:target:pt-20"
        >
            { children() }
        </div>
    }
}