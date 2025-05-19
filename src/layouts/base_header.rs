use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn BaseHeader(
    title: String,
    description: String,
    image: Option<String>,
) -> impl IntoView {
    // Set default image if none is provided
    let image_url = image.unwrap_or_else(|| "/syakir.webp".to_string());

    // You can define your base site URL statically or via environment config
    let site_url = "https://dev-dhanushkumar.me"; // Replace with actual base URL
    let current_path = window().location().pathname().unwrap_or_default();
    let full_url = format!("{site_url}{current_path}");
    let og_image_url = format!("{site_url}{image_url}");

    view! {
        <>
            // Global metadata
            <Meta charset="utf-8" />
            <Meta name="viewport" content="width=device-width,initial-scale=1" />
            <Link rel="icon" href="/assets/img/favicon.png" /> // Adjust path accordingly
            <Meta name="generator" content="Leptos" />

            // Canonical URL
            <Link rel="canonical" href=full_url.clone() />

            // Primary Meta Tags
            <Title text=title.clone() />
            <Meta name="title" content=title.clone() />
            <Meta name="description" content=description.clone() />

            // Open Graph / Facebook
            <Meta property="og:type" content="website" />
            <Meta property="og:url" content=full_url.clone() />
            <Meta property="og:title" content=title.clone() />
            <Meta property="og:description" content=description.clone() />
            <Meta property="og:image" content=og_image_url.clone() />

            // Twitter
            <Meta property="twitter:card" content="summary_large_image" />
            <Meta property="twitter:url" content=full_url />
            <Meta property="twitter:title" content=title />
            <Meta property="twitter:description" content=description />
            <Meta property="twitter:image" content=og_image_url />
        </>
    }
}
