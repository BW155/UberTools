#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use lazy_static::lazy_static;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::collections::HashMap;

mod tools;

#[derive(Serialize, Clone)]
pub struct Tool {
    name: String,
    description: String,
    url: String,
}

impl Tool {
    pub fn new(name: &str, description: &str, url: &str) -> Self {
        Tool {
            name: name.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }
}

lazy_static! {
    static ref ALL_TOOLS: Vec<Tool> = vec![
        Tool::new(
            "String Word Count",
            "Counts Words In A Text",
            "/tools/strings/count",
        ),
        Tool::new("MD5 Hash", "Hash Text To MD5", "/tools/hashing/md5_hash",),
        Tool::new(
            "Bcrypt Hash",
            "Hash Text To Bcrypt",
            "/tools/hashing/bcrypt_hash",
        ),
        Tool::new(
            "Binary Conversion",
            "Convert Binary To For Example HEX",
            "/tools/binary/binary_conversion",
        )
    ];
}

#[get("/")]
fn index() -> Template {
    Template::render("index", HashMap::<String, String>::new())
}

#[get("/search/<query>")]
fn search(query: String) -> Json<Vec<Tool>> {
    let query = query.to_lowercase();
    Json(
        ALL_TOOLS
            .iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query)
                    || t.description.to_lowercase().contains(&query)
            })
            .map(|t| t.clone())
            .collect::<Vec<Tool>>(),
    )
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .mount(
            "/",
            routes![
                index,
                tools::hashing::md5::md5_hash,
                tools::hashing::md5::md5_hash_query,
                tools::hashing::bcrypt::bcrypt_hash,
                tools::hashing::bcrypt::bcrypt_hash_query,
                tools::strings::count::count_string,
                tools::binary::binary_conversion::binary_conversion,
                search,
            ],
        )
        .launch();
}
