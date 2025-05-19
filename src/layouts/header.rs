use leptos::prelude::*;
use log::info;
use crate::layouts::hyper_link::HyperLink;

// Define the menu items structure
struct MenuItem {
    link: &'static str,
    label: &'static str,
}

#[component]
pub fn Header() -> impl IntoView {
    // Create a signal for the theme state
    let (is_dark, set_is_dark) = signal(true);

    // Initialize the theme based on system preference and localStorage on client-side
    Effect::new(move |_| {
        // This code runs on the client side only
        {
            
            let window = window();
            info!("Window: {:?}", window);
            
            // Check if system prefers dark mode
            let prefers_dark = window
                .match_media("(prefers-color-scheme: dark)")
                .unwrap()
                .unwrap()
                .matches();
                
            // Check if theme is stored in localStorage
            let storage = window.local_storage().unwrap().unwrap();
            let stored_theme = storage.get_item("theme").unwrap();

            info!("Stored theme: {:?}", stored_theme);
            
            let theme = match stored_theme {
                Some(theme) => theme,
                None => {
                    if prefers_dark {
                        "dark".to_string()
                    } else {
                        "light".to_string()
                    }
                }
            };
            
            // Set the initial theme
            if theme == "dark" {
                set_is_dark.set(true);
                let document = window.document().unwrap();
                let root = document.document_element().unwrap();
                root.class_list().add_1("dark").unwrap();
            }
            
            // Store theme in localStorage
            storage.set_item("theme", &theme).unwrap();
        }
    });
    
    // Toggle theme function
    let toggle_theme = move |_| {
        set_is_dark.update(|is_dark| {
            *is_dark = !*is_dark;
        });

        info!("Toggling theme: {:?}", is_dark.get());
        
        {
            let window = window();
            let document = window.document().unwrap();
            let root = document.document_element().unwrap();
            
            if is_dark.get() {
                root.class_list().add_1("dark").unwrap();
                root.class_list().remove_1("light").unwrap();
                
                // Update giscus theme if present
                send_giscus_message("dark_dimmed");
            } else {
                root.class_list().remove_1("dark").unwrap();
                root.class_list().add_1("light").unwrap();
                
                // Update giscus theme if present
                send_giscus_message("light");
            }
            
            // Store theme in localStorage
            let storage = window.local_storage().unwrap().unwrap();
            storage.set_item("theme", if is_dark.get() { "dark" } else { "light" }).unwrap();
        }
    };
    
    // Define menu items
    let menu = vec![
        MenuItem { link: "/about/", label: "About" },
        MenuItem { link: "/posts/", label: "Posts" },
        MenuItem { link: "/projects/", label: "Projects" },
        MenuItem { link: "/tags/", label: "Tags" },
    ];

    view! {
        <header 
            id="site-header"
            class="flex items-center w-full transition duration-300 z-[999] border-[var(--soft-border-color)]"
        >
            <div class="@container mx-auto flex flex-col  justify-between w-full max-w-5xl px-4 sm:px-6 lg:px-8">
                <nav class="flex gap-4 items-center justify-between py-3">
                    <h2 class="m-0">
                        <a href="/" class="flex gap-2 items-center text-xl font-black uppercase">
                            <img 
                                src="/images/dev-dhanushkumar-small.jpg" 
                                alt="Your site name" 
                                class="rounded-full w-11 h-11 border-white border-2 shadow-lg"
                            />
                        </a>
                    </h2>
                    <div class="flex">
                        // GitHub icon - light mode
                        <a 
                            class="p-3" 
                            class:hidden={ move || is_dark.get() } 
                            href="https://github.com/dev-dhanushkumar" 
                            target="_blank"
                        >
                            <img src="/images/github-mark-white.svg" width="24" height="24" alt="Github logo" />
                        </a>
                        
                        // GitHub icon - dark mode
                        <a 
                            class="p-3" 
                            class:hidden={move || !is_dark.get()} 
                            href="https://github.com/dev-dhanushkumar" 
                            target="_blank"
                        >
                            <img src="/images/linkedin.svg" width="24" height="24" alt="Github logo" />
                        </a>
                        

                        // Theme toggle button
                        <button class="p-3 cursor-pointer" id="themeToggle" aria-label="Theme mode" on:click=toggle_theme>

                            <Show
                                when=move || is_dark.get()
                                fallback=move || view! {
                                    // Sun icon when `is_dark` is false
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="24"
                                        height="24"
                                        viewBox="0 0 24 24"
                                        fill="none"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                    >
                                        <circle cx="12" cy="12" r="4"></circle>
                                        <path d="M12 2v2"></path>
                                        <path d="M12 20v2"></path>
                                        <path d="m4.93 4.93 1.41 1.41"></path>
                                        <path d="m17.66 17.66 1.41 1.41"></path>
                                        <path d="M2 12h2"></path>
                                        <path d="M20 12h2"></path>
                                        <path d="m6.34 17.66-1.41 1.41"></path>
                                        <path d="m19.07 4.93-1.41 1.41"></path>
                                    </svg>
                                }
                            >
                                // Moon icon when `is_dark` is true
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="24"
                                    height="24"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
                                </svg>
                            </Show>
                        </button>
                    </div>
                </nav>
                
                // Navigation menu in the center of page
                <div class="fixed top-3 w-auto max-w-full px-3 left-1/2 -translate-x-1/2 z-[999]">
                    <div class="flex px-2.5 bg-white/90 dark:bg-zinc-900/75 backdrop-blur-md leading-tight rounded-full border dark:border-zinc-700">
                        // Home link
                        <div class="px-2.5 text-sm">
                            <HyperLink href="/".to_string() class="py-3 flex items-center".to_string()>
                                <svg 
                                    xmlns="http://www.w3.org/2000/svg" 
                                    width="20" 
                                    height="20" 
                                    viewBox="0 0 24 24" 
                                    fill="none" 
                                    stroke="currentColor" 
                                    stroke-width="1.7" 
                                    stroke-linecap="round" 
                                    stroke-linejoin="round" 
                                    class="lucide lucide-house"
                                >
                                    <path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/>
                                    <path d="M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
                                </svg>
                            </HyperLink>
                        </div>
                        
                        // Menu items
                        {menu.into_iter().map(|item| {
                            view! {
                                <div class="px-2.5 text-sm">
                                    <HyperLink href={item.link} class="py-3 flex items-center".to_string()>
                                            {item.label}
                                    </HyperLink>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </header>
    }
}



// Utility function to send messages to Giscus iframe
fn send_giscus_message(theme: &str) {
    use leptos::wasm_bindgen::JsValue;
    use leptos::web_sys::{window, Element};
    use leptos::wasm_bindgen::JsCast;
    
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Ok(Some(iframe)) = document.query_selector("iframe.giscus-frame") {
                let message = js_sys::Object::new();
                let giscus = js_sys::Object::new();
                let config = js_sys::Object::new();
                
                js_sys::Reflect::set(&config, &JsValue::from_str("theme"), &JsValue::from_str(theme)).unwrap();
                js_sys::Reflect::set(&giscus, &JsValue::from_str("setConfig"), &config).unwrap();
                js_sys::Reflect::set(&message, &JsValue::from_str("giscus"), &giscus).unwrap();
                
                let iframe_element: &Element = &iframe;
                
                if let Some(content_window) = iframe_element.dyn_ref::<leptos::web_sys::HtmlIFrameElement>().unwrap().content_window() {
                    content_window.post_message(&message, "https://giscus.app").unwrap();
                }
            }
        }
    }
}