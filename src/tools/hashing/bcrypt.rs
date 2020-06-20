use bcrypt;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/tools/hashing/bcrypt_hash")]
pub fn bcrypt_hash() -> Template {
    let data = HashMap::<&str, String>::new();
    Template::render("tools/hashing/bcrypt_hash", &data)
}

#[get("/tools/hashing/bcrypt_hash?<text>&<cost>")]
pub fn bcrypt_hash_query(text: String, cost: u32) -> Template {
    let mut data = HashMap::<&str, String>::new();

    if let Ok(hash) = bcrypt::hash(&text, cost) {
        data.insert("hash", hash);
    }
    data.insert("original", text);
    data.insert("original_cost", format!("{}", cost));

    Template::render("tools/hashing/bcrypt_hash", &data)
}
