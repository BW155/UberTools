use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/tools/images/resize")]
pub fn image_resize() -> Template {
    let data = HashMap::<&str, String>::new();
    Template::render("tools/images/resize", &data)
}
