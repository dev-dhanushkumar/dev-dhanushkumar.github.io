use leptos::prelude::*;

#[component]
pub fn AboutTheAuthor() -> impl IntoView {
    view! {
        <div class="about-the-author py-4 border-b">
            <h3 class="font-extrabold uppercase mb-4 text-sm">"About the Author"</h3>
            <div class="flex items-center gap-4 mb-3">
                <img
                    src="/images/dev-dhanushkumar-small.jpg"
                    alt="Author avatar"
                    class="rounded-full w-12 h-12 object-cover"
                />
                <div>
                    <div class="font-semibold">"Dhanush Kumar"</div>
                    <div class="text-xs text-zinc-500">"Software Developer from India"</div>
                </div>
            </div>
            <div class="author-desc text-[.825rem] my-4 text-zinc-700 dark:text-zinc-300">
                <p>
                    "I'm a passionate software developer from India, specializing in efficient and high-performance software development from scratch. I enjoy researching, building scalable APIs and web servers, and sharing my knowledge on "
                    <a class="text-blue-600 underline" href="https://dev.to/dev-dhanushkumar" target="_blank">"dev.to"</a>
                    " and "
                    <a class="text-blue-600 underline" href="https://stackoverflow.com/users/23688025/dev-dhanushkumar" target="_blank">"stack overflow"</a>
                    "."
                </p>
            </div>
        </div>
    }
}
