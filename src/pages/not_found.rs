use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="py-8 min-h-[calc(100vh-160px)]">
            <p class="text-5xl text-center font-bold my-4">"404 Page Not Found"</p>
		</div>
    }
}