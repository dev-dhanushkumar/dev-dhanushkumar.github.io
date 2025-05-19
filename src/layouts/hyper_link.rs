// use leptos::prelude::*;
// use leptos_router::hooks::use_location;
// use leptos::attr::list;

// #[component]
// pub fn HyperLink(
//     href: String,
//     class: String,
//     children: Children,
// ) -> impl IntoView {
//     // grab the current path from the router
//     let location = use_location();
//     // memoize it so we re-compute only when it changes
//     let current_path = Memo::new(move |_| location.pathname.get());

//     // active if equal to href or href + "/"
//     let is_active = move || {
//         let p = current_path.get();
//         p == href || p == format!("{}/", href)
//     };

//     view! {
//         <a
//             href=href.clone()
//             // build up a single class string from several bits
//             class=list([
//                 class.clone(),
//                 // bottom border when active
//                 if is_active() { "border-b border-zinc-500 dark:border-zinc-300".to_string() } else { "None".to_string() },
//                 // inactive text styling
//                 if !is_active() { "text-zinc-600 dark:text-zinc-200 hover:text-zinc-900 dark:hover:text-zinc-50".to_string() } else { "None".to_string() },
//                 // active text color
//                 if is_active() { "text-zinc-950 dark:text-zinc-50".to_string() } else { "None".to_string() },
//             ])
//         >
//             { children() }
//         </a>
//     }
// }



use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_router::components::A;

#[component]
pub fn HyperLink(
    #[prop(into)] href: String,
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    // 1) track current path
    let location    = use_location();
    let current_path = Memo::new(move |_| location.pathname.get());
    // 2) derive active flag
    let link = href.clone();
    let is_active = move || {
        let p = current_path.get();
        p == link.clone() || p == format!("{}/", link.clone())
    };
    // 3) build up one combined class‐string
    let class_name = Memo::new(move |_| {
        let mut classes = Vec::new();
        // base class if provided
        if let Some(base) = &class {
            if !base.is_empty() {
                classes.push(base.clone());
            }
        }
        if is_active() {
            classes.push("border-b border-zinc-500 dark:border-zinc-300".to_string());
            classes.push("text-zinc-950 dark:text-zinc-50".to_string());
        } else {
            classes.push("text-zinc-600 dark:text-zinc-200 hover:text-zinc-900 dark:hover:text-zinc-50".to_string());
        }
        classes.join(" ")
    });

    view! {
        <div class=class_name>
            <A href=href.clone()>
                { children() }
            </A>
        </div>
    }
}