use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use crate::layouts::{base_header::BaseHeader, header::Header};
use crate::pages::{home::Home, not_found::NotFound, about::About, projects::Projects, posts::Posts, tags::Tags};

mod layouts;
mod pages;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang = "en" attr:dir = "ltr" attr:data-theme = "dark" />
        
        <Title text="Welcome to leptos CSR" />
        <link data-trunk rel = "copy-dir" href = "/public/images" />
        <BaseHeader 
            title = "dev-dhanushkumar-portfolio".to_string()
            description = "Welcome to my portfolio".to_string()
            image = Some("/syakir.webp".to_string())
        />
        <Meta charset="UTF-8" />
        <Meta name = "viewpoint" content = "width=device-width, inital-scale = 1.0" />
        <Link rel="stylesheet" href="/tailwind.css" />
        


        <body class="bg-white dark:bg-zinc-900 dark:text-zinc-100 pt-16 sm:pt-0">
        <Router>
            <Header />
            <main>
                <Routes fallback = || view! {<NotFound/>}>
                    <Route path = path!("/") view = Home />
                    <Route path = path!("/about") view = About />
                    <Route path = path!("/projects") view = Projects />
                    <Route path = path!("/posts") view = Posts />
                    <Route path = path!("/tags") view = Tags />
                </Routes>
            </main>
        </Router>
        </body>
    }
}

