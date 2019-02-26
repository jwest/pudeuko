use serde_json::json;
use reqwest::{Client, ClientBuilder};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use crate::domain::{ItemList};

const DROPBOX_FILE_PATH: &'static str = "/pudeuko/data.json";
const DROPBOX_DOWNLOAD_URL: &'static str = "https://content.dropboxapi.com/2/files/download";
const DROPBOX_UPLOAD_URL: &'static str = "https://content.dropboxapi.com/2/files/upload";

pub struct DropboxClient {
    client: Client,
    download_headers: HeaderMap,
    upload_headers: HeaderMap,
}

impl DropboxClient {
    pub fn new(dropbox_token: String) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(AUTHORIZATION, format!("Bearer {}", dropbox_token).parse().unwrap());

        let mut download_headers = HeaderMap::new();
        download_headers.insert(
            "Dropbox-API-Arg",
            json!({ "path": DROPBOX_FILE_PATH }).to_string().parse().unwrap(),
        );

        let mut upload_headers = HeaderMap::new();
        upload_headers.insert(CONTENT_TYPE, "application/octet-stream".parse().unwrap());
        upload_headers.insert(
            "Dropbox-API-Arg",
            json!({ "path": DROPBOX_FILE_PATH, "mode": "overwrite" }).to_string().parse().unwrap(),
        );

        Self {
            client: ClientBuilder::new().default_headers(default_headers).build().unwrap(),
            download_headers,
            upload_headers,
        }
    }

    pub fn fetch(self: &Self) -> ItemList {
        let body = self.client.get(DROPBOX_DOWNLOAD_URL)
            .headers(self.download_headers.clone())
            .send().unwrap()
            .text().unwrap();

        let items: ItemList = serde_json::from_str(&body).unwrap();
        items
    }

    pub fn upload(self: &Self, list: &ItemList) {
        let json = serde_json::to_string(list).unwrap();

        self.client.post(DROPBOX_UPLOAD_URL)
            .headers(self.upload_headers.clone())
            .body(json)
            .send().unwrap();
    }
}
