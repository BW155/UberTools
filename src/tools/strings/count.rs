use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/tools/strings/count")]
pub fn count_string() -> Template {
    Template::render("tools/strings/count", HashMap::<String, String>::new())
}
