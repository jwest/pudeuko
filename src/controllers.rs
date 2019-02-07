use rocket::{self, get, post};
use rocket_contrib::json::{Json};
use crate::domain::{ItemList, ContentDTO};
use crate::dropbox_client;

#[get("/")]
pub fn get_items() -> String {
    let list: ItemList = dropbox_client::fetch_pudeuko();
    let json = serde_json::to_string(&list).unwrap();
    json
}

#[post("/", format = "application/json", data = "<content>")]
pub fn post_item(content: Json<ContentDTO>) -> String {
    "".to_string()
}
