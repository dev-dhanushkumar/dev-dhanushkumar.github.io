use deunicode::deunicode;

pub fn slugify(input: &str) -> String {
    let mut slug = deunicode(input).to_lowercase();
    slug = slug.replace(|c: char| !c.is_ascii_alphanumeric() && !c.is_whitespace(), " ");
    slug = slug.split_whitespace().collect::<Vec<_>>().join("-");
    slug
}
