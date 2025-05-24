use deunicode::deunicode;

pub fn slugify(input: &str) -> String {
    let mut slug = deunicode(input).to_lowercase();
    slug = slug.replace(|c: char| !c.is_ascii_alphanumeric() && !c.is_whitespace(), " ");
    slug = slug.split_whitespace().collect::<Vec<_>>().join("-");
    slug
}


pub fn unslugify(slug: &str) -> String {
    slug.replace("-", " ").split(' ').map(|s| {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }).collect::<Vec<_>>().join(" ")
}
