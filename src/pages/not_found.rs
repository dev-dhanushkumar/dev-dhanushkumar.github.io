use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="@container text-center mx-auto flex flex-col  justify-between w-full max-w-6xl px-4 sm:px-6 lg:px-8">
            <img width={300} class="my-8 mx-auto" src="/images/404-not-found.png" alt="404 Page Not Found" />
            <div class="p-4">
                <h1 class="text-5xl font-bold my-4">Page Not Found</h1>
                <p class="text-xl">
                    "The page you're looking for is either moved or no longer exists." <br />
                    "Please contact the site owner that linked you to the original URL and let them know their link is broken."
                </p>
            </div>
        </div>
    }
}