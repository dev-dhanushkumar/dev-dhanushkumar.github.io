use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use include_dir::{include_dir, Dir};
// use log::info;
// use std::path::Path;

static BLOG_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/content/blog");

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPost {
    pub title: String,
    pub seo_title: String,
    pub slug: String,
    pub description: String,
    pub pub_date: NaiveDate,
    pub updated_date: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
    pub cover_image: Option<String>,
}


pub fn load_blog_posts() -> Vec<BlogPost> {
    let mut posts = Vec::new();

    for entry in BLOG_DIR.entries() {
      if let Some(dir) = entry.as_dir() {
        let slug = dir.path().file_name().unwrap().to_str().unwrap();
        // info!("Found blog post directory: {:?}", dir.path());
        let index_path = format!("{slug}/index.md");
        if let Some(file) = BLOG_DIR.get_file(&index_path) {
          let content = file.contents_utf8().unwrap();
          if let Some(post) = parse_post_metadata(content, dir.path().file_name().unwrap().to_str().unwrap()) {
            posts.push(post);
          }
        }
      }
    }

    posts.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
    posts
}

pub fn get_blog_post_by_slug(slug: &str) -> Option<BlogPost> {
    let index_path = format!("{slug}/index.md");
    BLOG_DIR.get_file(&index_path).and_then(|file| {
        let content = file.contents_utf8()?;
        parse_post_metadata(content, slug)
    })
}



fn parse_post_metadata(content: &str, slug: &str) -> Option<BlogPost> {
    let re = regex::Regex::new(r#"(?s)^---\r?\n(.*?)\r?\n---"#).ok()?;
    let caps = re.captures(content)?;
    let yaml_str = caps.get(1)?.as_str();

    // info!("YAML content:\n{}", yaml_str);

    let mut post: BlogPost = serde_yaml::from_str(yaml_str)
        .map_err(|e| log::error!("YAML parse error for slug {slug}: {e}"))
        .ok()?;

    post.slug = slug.to_string();
    Some(post)
}


pub fn strip_front_matter(md: &str) -> String {
    let mut lines = md.lines();
    // must start with `---`
    if lines.next().map(str::trim) != Some("---") {
        return md.to_string();
    }
    // skip until the next `---`
    for line in &mut lines {
        if line.trim() == "---" {
            // everything after this line is the body
            return lines.collect::<Vec<_>>().join("\n");
        }
    }
    // no closing marker → nothing left
    String::new()
}