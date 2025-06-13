pub mod token;

use std::fs::File;
use std::io::{self, Cursor, Write};
use std::path::Path;

use reqwest::header::{HeaderMap, CONTENT_DISPOSITION};
use reqwest::{Client, Response, Url};
use serde_json::json;
use zip::ZipArchive;

use token::NAIAccessToken;

use crate::preset::ImagePreset;
use crate::utils::constants::{
    NAI_IMG_GEN_ENDPOINT, NAI_LOGIN_ENDPOINT, NAI_ORIGIN, NAI_REFERER, USER_AGENT,
};
use crate::utils::hash::encode_access_key;

const IMAGE_OUTPUT_DIR: &'static str = "./generated/";

#[derive(Clone, Debug)]
pub struct NAIClient {
    pub client: Client,
    pub header: HeaderMap,
    pub token: NAIAccessToken,
}

impl NAIClient {
    pub async fn new(username: &str, password: &str) -> Self {
        // initializaing headers
        let mut header = HeaderMap::new();
        header.insert("Content-Type", "application/json".parse().unwrap());
        header.insert("Origin", NAI_ORIGIN.parse().unwrap());
        header.insert("Referer", NAI_REFERER.parse().unwrap());
        header.insert("User-Agent", USER_AGENT.parse().unwrap());

        let client = Client::new();
        let key = encode_access_key(&username, &password);

        let request = client
            .post(Url::parse(NAI_LOGIN_ENDPOINT).unwrap())
            .headers(header.clone())
            .json(&json!({
                "key": key,
            }))
            .build()
            .unwrap();

        let response = client.execute(request).await.unwrap();
        let token: NAIAccessToken = response.json().await.unwrap();

        Self {
            client,
            header,
            token,
        }
    }

    pub async fn send_request(self, preset: ImagePreset) -> Response {
        let request = self
            .client
            .post(Url::parse(NAI_IMG_GEN_ENDPOINT).unwrap())
            .bearer_auth(self.token)
            .headers(self.header)
            .json(&json!({
                "input": preset.prompt,
                "model": preset.model,
                "parameters": preset.parameters,
            }))
            .build()
            .unwrap();

        self.client.execute(request).await.unwrap()
    }

    pub async fn generate_image(self, preset: ImagePreset) {
        let response = self.send_request(preset).await;

        let content_disposition = response
            .headers()
            .get(CONTENT_DISPOSITION)
            .unwrap()
            .to_str()
            .unwrap();

        let parse_filename = |s: &str| -> Option<String> {
            for part in s.split(';') {
                if let Some(filename) = part.trim().strip_prefix("filename=") {
                    return Some(filename.trim_matches('"').to_string());
                }
            }
            None
        };

        let filename = parse_filename(content_disposition).unwrap();
        let bytes = response.bytes().await.unwrap();
        let cursor = Cursor::new(&bytes);

        // save zip file itself to the output directory
        let zip_path = Path::new(IMAGE_OUTPUT_DIR).join(filename);
        let mut zip = File::create(zip_path).unwrap();
        zip.write_all(&bytes).unwrap();

        // clear output directory
        // for entry in fs::read_dir(IMAGE_OUTPUT_DIR).unwrap() {
        //     fs::remove_file(entry.unwrap().path()).unwrap();
        // }

        let mut archive = ZipArchive::new(cursor).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let output_path = Path::new(IMAGE_OUTPUT_DIR).join(file.name());

            let mut output_file = File::create(&output_path).unwrap();
            io::copy(&mut file, &mut output_file).unwrap();
        }
    }
}
