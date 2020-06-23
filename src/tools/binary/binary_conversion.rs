use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/tools/binary/binary_conversion")]
pub fn binary_conversion() -> Template {
    Template::render(
        "tools/binary/binary_conversion",
        HashMap::<String, String>::new(),
    )
}
