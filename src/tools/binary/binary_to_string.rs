use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/tools/binary/binary_to_string")]
pub fn binary_to_string() -> Template {
    Template::render(
        "tools/binary/binary_to_string",
        HashMap::<String, String>::new(),
    )
}
