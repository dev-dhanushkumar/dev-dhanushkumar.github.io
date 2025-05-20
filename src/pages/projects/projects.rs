use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub description: String,
    pub demo_link: String,
    pub post_link: Option<String>,
    pub tags: Option<Vec<String>>,
}


pub fn load_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Spring-Boot-SignUp-Login-API".to_string(),
            description: "Signup and Login simple Authentication API in spring boot".to_string(),
            demo_link: "https://github.com/dev-dhanushkumar/Spring-Signup-Login-API".to_string(),
            post_link: None,
            tags: Some(vec!["Spring-Boot".to_string()]),
        },
        Project {
            name: "Rust-WARP-Cookie-Based-Authentication".to_string(),
            description: "Get Current user information using Cookies".to_string(),
            demo_link: "https://github.com/dev-dhanushkumar/Rust-WARP-Cookie-Based-Authentication".to_string(),
            post_link: None,
            tags: Some(vec!["Rust".to_string()]),
        },
        Project {
            name: "Leptos-Dashboard-App".to_string(),
            description: "Full-stack Rust Dashboard App with Rust🦀! Leveraging Leptos, Actix Web and SurrealDB!".to_string(),
            demo_link: "https://github.com/dev-dhanushkumar/leptos_dashboard_app".to_string(),
            post_link: None,
            tags: Some(vec!["Leptos".to_string(), "Rust".to_string()]),
        },
        Project {
            name: "Go-BookStore-API".to_string(),
            description: "To create book API operations using Golang".to_string(),
            demo_link: "https://github.com/dev-dhanushkumar/Go-BookStore-API".to_string(),
            post_link: None,
            tags: Some(vec!["Golang".to_string(), "REST API".to_string()]),
        },
        Project {
            name: "JS-WebRTC-Video-Chat".to_string(),
            description: "JS WebRTC Video Chat App".to_string(),
            demo_link: "https://github.com/dev-dhanushkumar/JS-Small-Projects/tree/main/js-WebRTC-Video-Chat".to_string(),
            post_link: None,
            tags: Some(vec!["Javascript".to_string()]),
        },
        Project {
            name: "Golang-ToDo-List-CLI-Tool".to_string(),
            description: "Command-line tool designed to help you manage your daily tasks efficiently".to_string(),
            demo_link: "https://github.com/dev-dhanushkumar/Golang-Projects/tree/main/golang_task".to_string(),
            post_link: Some("https://dev.to/dev-dhanushkumar/mytask-todo-cli-tool-2kej".to_string()),
            tags: Some(vec!["CLI".to_string(), "Golang".to_string()]),
        },
    ]
}