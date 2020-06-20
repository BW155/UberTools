use md5;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/tools/hashing/md5_hash")]
pub fn md5_hash() -> Template {
    let data = HashMap::<&str, String>::new();
    Template::render("tools/hashing/md5_hash", &data)
}

#[get("/tools/hashing/md5_hash?<text>")]
pub fn md5_hash_query(text: String) -> Template {
    let mut data = HashMap::<&str, String>::new();

    let hash = format!("{:x}", md5::compute(&text));
    data.insert("hash", hash);
    data.insert("original", text);

    Template::render("tools/hashing/md5_hash", &data)
}
