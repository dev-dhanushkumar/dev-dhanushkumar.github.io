use leptos::prelude::*;
use crate::{components::{prose::Prose, post_item::PostItem, project_list::ProjectList}, content::blog::load_blog_posts, pages::projects::projects::load_projects};

#[component]
pub fn Home() -> impl IntoView {
    let posts = load_blog_posts().into_iter().take(5).collect::<Vec<_>>();
    let top6Projects = load_projects().into_iter().take(6).collect::<Vec<_>>();

    view! {
        <div class="@container mx-auto flex flex-col  justify-between w-full max-w-5xl px-4 sm:px-6 lg:px-8">
            <div class="w-full md:flex md:items-center gap-16 mb-10">
                <div class="text-pretty leading-9">
                    <h1 class="text-3xl font-bold mb-4">"Hi There 👋, Dhanush Kumar!" </h1>
                    <Prose>
                        <div class="text-lg">
                            <p>"I'm a passionate software developer from India, specializing in efficient and high-performance software development 
                            from scratch. I enjoy researching, building scalable APIs and web servers, and sharing my knowledge on" <a href="https://dev.to/dev-dhanushkumar" target="_blank">"dev.to"</a>  "and" 
                            <a href="https://stackoverflow.com/users/23688025/dev-dhanushkumar" target="_blank">"stack overflow"</a>. 
                            </p><br/>
                            <p>"Proficient in Rust, Java, Golang, and JavaScript, I focus on writing optimized and secure code that 
                            enhances system performance. I also have expertise in Docker, Kubernetes, and DevOps, enabling me to build scalable, 
                            secure, and optimized applications."
                            </p>  
                        </div>
                    </Prose>
                </div>
                <img src="/images/dev-dhanushkumar.jpg" alt="Dhanush Kumar" width={300}  loading="eager" class="rounded-3xl h-40px rotate-3 mx-4 md:mx-0"/>

            </div>


            <main>
                <div class="mt-6 mb-12">
                    <div class="flex justify-between gap-2 border-b mb-1 dark:border-b-zinc-700">
                        <h2 class="text-lg font-bold mb-3">"Recent Posts"</h2>
                        <a href="/posts/" class="inline-block py-2 underline dark:prose-invert">"All posts »"</a>
                    </div>
                    <div>
                        {posts.into_iter().map(|post| view! { <PostItem post={post}/> }).collect_view()}
                    </div>
                    <div class="text-right">
                    </div>
                    
                    <div class="mt-12">
                        <div class="flex justify-between gap-2 mb-1">
                        <h2 class="text-lg font-bold mb-3">My Projects</h2>
                        <a href="/projects/" class="inline-block py-2 underline dark:prose-invert">"All projects »"</a>
                        </div>
                        <ProjectList projects={top6Projects}/>
                    </div>
                </div>
            </main>
        </div>
    }
}